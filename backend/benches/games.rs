use criterion::{black_box, Criterion};
use futures::executor::block_on;
use secret_clan::model::Game;
use std::time::Instant;
use task::LocalSet;
use tokio::task;

pub async fn bench_games(c: &mut Criterion, local: &LocalSet) {
    let mut db_group = c.benchmark_group("games");
    db_group.bench_function("start", |b| {
        b.iter_custom(|iters| {
            let t = local.run_until(async {
                let start = Instant::now();
                for _ in 0..iters {
                    let mut game = Game::new(black_box("game"), black_box("TOKEN"));
                    game.add_player(black_box("player_a"));
                    game.add_player(black_box("player_b"));
                    game.add_player(black_box("player_c"));
                    game.add_player(black_box("player_d"));
                    game.add_player(black_box("player_e"));
                    game.add_player(black_box("player_f"));
                    game.start();
                }
                start.elapsed()
            });

            block_on(t)
        });
    });
}
