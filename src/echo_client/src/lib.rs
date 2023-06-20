use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};

/// split은 AsyncRead + AsyncWrite를 구현하면 사용할 수 있다.
///
/// io::split Mutex와 Arc로 이루어져 있어 오버헤드가 있다. 이를 피하려면 TcpStream::split을 이용해라
pub async fn io_split() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut rd, mut wr) = io::split(socket);

    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}

/// TcpStream은 참조를 이용하기에 zero-cost며 split()을 한 곳과 동일한 task내에 있어야 한다. ( Arc, Mutex를 사용하지 않음 )
pub async fn tcp_stream_split() -> io::Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;

    tokio::spawn(async move {
        let (mut rd, mut wr) = socket.split();

        if io::copy(&mut rd, &mut wr).await.is_err() {
            eprintln!("failed to copy");
        }
    });

    Ok(())
}