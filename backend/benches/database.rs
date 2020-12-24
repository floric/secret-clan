use criterion::{black_box, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion};
use futures::executor::block_on;
use rand::{distributions::Alphanumeric, prelude::*};
use rand_pcg::Pcg64;
use secret_clan::{model::Player, server::app_context::AppContext};
use std::time::Instant;
use task::LocalSet;
use tokio::task;

pub async fn bench_database(c: &mut Criterion, ctx: &AppContext, local: &LocalSet) {
    let mut db_group = c.benchmark_group("database");
    db_group.bench_function("persist", |b| {
        b.iter_custom(|iters| {
            let t = local.run_until(async {
                let _ = ctx.db().players().purge().await;
                let start = Instant::now();
                for _ in 0..iters {
                    let player = Player::new("game");
                    let _ = ctx.db().players().persist(black_box(&player)).await;
                }
                start.elapsed()
            });

            block_on(t)
        });
    });

    db_group.bench_function("get", |b| {
        b.iter_custom(|iters| {
            let t = local.run_until(async {
                let _ = ctx.db().players().purge().await;
                let player = Player::new("game");
                let _ = ctx.db().players().persist(&player).await;

                let start = Instant::now();
                let id = player.id();
                for _ in 0..iters {
                    let _ = ctx.db().players().get(black_box(id)).await;
                }
                start.elapsed()
            });

            block_on(t)
        });
    });

    bench_scan_with_sizes(&mut db_group, ctx, local, vec![10, 100, 1000, 10000]);
}

fn bench_scan_with_sizes(
    db_group: &mut BenchmarkGroup<WallTime>,
    ctx: &AppContext,
    local: &LocalSet,
    sizes: Vec<u32>,
) {
    for size in sizes {
        db_group.bench_with_input(BenchmarkId::new("scan", size), &size, |b, player_count| {
            b.iter_custom(|iters| {
                let t = local.run_until(async {
                    let _ = ctx.db().players().purge().await;
                    let mut rng = Pcg64::seed_from_u64(123);
                    for _ in 0..*player_count {
                        let random_name =
                            String::from_utf8(vec![rng.sample(Alphanumeric)]).unwrap();
                        let player = Player::new(&random_name);
                        let _ = ctx.db().players().persist(&player).await;
                    }

                    let start = Instant::now();
                    for _i in 0..iters {
                        let _ = ctx
                            .db()
                            .players()
                            .scan(Box::new(|p| p.name().starts_with("a")))
                            .await;
                    }
                    start.elapsed()
                });

                block_on(t)
            });
        });
    }
}
