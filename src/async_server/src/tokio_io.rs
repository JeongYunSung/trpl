//! Tokio의 I/O 연산은 표준 I/O와 동일하나 비동기적으로 동작한다.
//! I/O는 AsyncRead와 AsyncWriter 트레잇을 구현한다 예로 TcpStream, File, Stdout가 있다.
//! Tokio은 std I/O과 비슷한 async I/O를 제공한다 ex. copy

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

/// AsyncRead, AsyncWrite는 사용하기 쉬운 비동기 I/O 바이트 스트림을 제공한다.
/// 또한 이들의 metohd는 직접적으로 호출되진 않고 Future에서 poll을 통해 불려진다.
pub async fn async_io() -> io::Result<()> {
    let mut f = File::open("./async_server/src/foo.txt").await?;
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

pub async fn async_io_read_all() -> io::Result<()> {
    let mut f = File::open("async.txt").await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", buffer);
    Ok(())
}

pub async fn write() -> io::Result<()> {
    let mut file = File::create("async.txt").await?;

    let n = file.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'shome bytes'.", n);
    Ok(())
}