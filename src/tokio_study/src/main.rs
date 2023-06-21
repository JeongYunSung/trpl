#[tokio::main]
async fn main() {
    tokio::select! {
        _ = async {} => {
            println!("1");
        }
        _ = async {} => {
            println!("2");
        }
    }
}