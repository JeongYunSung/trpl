use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// Ok(0)은 remote에 있는 socket이 closed됐다는 의미이다. 쉽게 말해 EOF
///
/// TcpStream의 경우 read()에서 Ok(0)를 준다면 return으로 꼭 끝내주어야 한다.
///
/// 만일 계속 read()를 값을 즉시 반환하는데 이 경우 무한루프를 매우 빠르게 돌리므로 CPU사용률 100%퍼를 확인 할 것이다.
pub async fn open_server() -> io::Result<()>  {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[0..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}