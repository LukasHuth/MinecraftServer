#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fast_server::server::Server::start().await
}
