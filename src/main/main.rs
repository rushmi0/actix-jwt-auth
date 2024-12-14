mod app;
mod module;
mod services;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
