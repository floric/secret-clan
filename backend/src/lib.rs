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
    let rt = Builder::new_multi_thread()
        .thread_name("sc")
        .build()
        .expect("Creating runtime failed");

    rt.block_on(async {
        let (ctx, mut changes) = AppContext::init_with_changes();
        let ctx: &'static AppContext = Box::leak(Box::new(ctx));

        init_jobs(ctx);

        tokio::spawn(async move {
            changes.start_listening(&ctx).await;
        });

        run_server(&ctx).await;
    });
}
