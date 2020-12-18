mod config;
mod db;
mod jobs;
mod model;
mod server;

use jobs::init_jobs;
use server::{app_context::AppContext, run_server};
use tokio::runtime::Builder;

extern crate chrono;
extern crate envconfig;
extern crate log;

fn main() {
    let mut rt = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .thread_name("threadpool")
        .build()
        .expect("Creating runtime failed");

    rt.block_on(async {
        let ctx: &'static AppContext = Box::leak(Box::new(AppContext::init()));

        init_jobs(ctx);

        run_server(&ctx).await;
    });
}
