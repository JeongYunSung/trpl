use mini_redis::Frame;
use tokio::net::TcpStream;
use framing::Connection;

#[tokio::main]
async fn main() {
    let socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let mut connection = Connection::new(socket);

    connection.write_frame(&Frame::Simple("PING".to_string())).await.unwrap();
}