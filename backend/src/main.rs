use server::run_server;

mod config;
mod model;
mod persistence;
mod server;

#[tokio::main]
async fn main() {
    run_server().await;
}
