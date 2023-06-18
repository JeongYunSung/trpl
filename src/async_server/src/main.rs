use async_server::start_server;

/// 런타임시점에 main을 호출하고 프로그램을 진행해야 하는데, main함수가 async면 호출되자마자 종료되기에 사용할 수 없음
/// #[tokio::main]은 tokio에서 제공하는 매크로로 async_server func에 붙일 시 자동으로 blocking을 처리해줌
#[tokio::main]
async fn main() {
    start_server().await;
}
