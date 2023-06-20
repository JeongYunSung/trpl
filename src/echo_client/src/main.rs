use echo_client::tcp_stream_split;

#[tokio::main]
async fn main() {
   let _ = tcp_stream_split().await;
}