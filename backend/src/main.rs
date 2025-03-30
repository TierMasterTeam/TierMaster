use backend::core::app::App;

#[tokio::main]
async fn main() {
    App::run().await
}