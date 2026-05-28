mod server;

#[async_std::main]
async fn main() {
    println!("Starting server at http://127.0.0.1:8888");
    server::start_server().await;
}
