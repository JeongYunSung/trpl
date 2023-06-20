use std::io::Cursor;
use bytes::{Buf, BytesMut};
use tokio::net::TcpStream;
use mini_redis::{Frame, Result, frame::Error::Incomplete};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// 내부 버퍼가 frame에 부합한지 확인
    ///
    /// 부합하지 않다면 stream으로 부터 read한 데이터를 버퍼에 저장
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame))
            };

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None)
                } else {
                    return Err("connection reset by peer".into())
                }
            }
        }
    }

    /// Cursor를 open하고 내부 버퍼 데이터를 Frame에 해당하는지 Check
    ///
    /// 이 후 Cursor를 초기화하고 Frame을 Parse
    ///
    /// 원본 buffer를 Cursor의 position만큼 앞으로 전진
    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;

                buf.set_position(0);

                let frame = Frame::parse(&mut buf)?;

                self.buffer.advance(len);

                Ok(Some(frame))
            }

            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// 내부 버퍼에 데이터를 쓰고 버퍼가 꽉 찰 시 flush를 진행 함
    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unimplemented!(),
        }

        let _ = self.stream.flush().await;

        Ok(())
    }

    async fn write_decimal(&mut self, val: u64) -> io::Result<()> {
        use std::io::Write;

        let mut buf = [0u8; 12];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}