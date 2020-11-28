mod config;
mod model;
mod persistence;
mod server;

use server::{app_context::AppContext, run_server};

extern crate chrono;
extern crate envconfig;
extern crate log;

#[tokio::main]
async fn main() {
    let ctx: &'static AppContext = Box::leak(Box::new(AppContext::init()));
    run_server(&ctx).await;
}
