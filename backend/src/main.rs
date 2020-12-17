mod config;
mod jobs;
mod model;
mod persistence;
mod server;

use jobs::init_jobs;
use server::{app_context::AppContext, run_server};

extern crate chrono;
extern crate envconfig;
extern crate log;

#[tokio::main]
async fn main() {
    let ctx: &'static AppContext = Box::leak(Box::new(AppContext::init()));

    init_jobs();

    run_server(&ctx).await;
}
