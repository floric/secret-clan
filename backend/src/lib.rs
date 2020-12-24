use jobs::init_jobs;
use server::{app_context::AppContext, run_server};
use tokio::runtime::Builder;

mod config;
pub mod db;
pub mod jobs;
pub mod model;
pub mod server;

extern crate chrono;
extern crate envconfig;
extern crate log;

#[macro_use]
extern crate derivative;

pub fn run_app() {
    let mut rt = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .thread_name("sc")
        .build()
        .expect("Creating runtime failed");

    rt.block_on(async {
        let ctx: &'static AppContext = Box::leak(Box::new(AppContext::init()));

        init_jobs(ctx);

        run_server(&ctx).await;
    });
}
