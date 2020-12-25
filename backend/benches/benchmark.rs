mod database;
mod games;

use self::{database::bench_database, games::bench_games};
use criterion::{criterion_group, criterion_main, Criterion};
use secret_clan::server::app_context::AppContext;
use tokio::{runtime::Builder, task};

fn criterion_benchmark(c: &mut Criterion) {
    std::env::set_var("LOG_LEVEL", "warn");

    let mut rt = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .thread_name("sc")
        .build()
        .expect("Creating runtime failed");

    rt.block_on(async {
        let ctx: &'static AppContext = Box::leak(Box::new(AppContext::init()));
        let local = task::LocalSet::new();

        bench_games(c, &local).await;
        bench_database(c, ctx, &local).await;
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
