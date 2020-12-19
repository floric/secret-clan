mod cleanup_games;
mod cleanup_players;

use self::cleanup_games::cleanup_games;
use self::cleanup_players::cleanup_players;
use crate::server::app_context::AppContext;
use clokwerk::{Scheduler, TimeUnits};
use std::{thread, time::Duration};

const JOB_INTERVAL: u32 = 60;

pub fn init_jobs(ctx: &'static AppContext) {
    tokio::task::spawn(async move {
        let mut scheduler = Scheduler::new();

        scheduler
            .every(JOB_INTERVAL.seconds())
            .run(cleanup_games(ctx));
        scheduler
            .every(JOB_INTERVAL.seconds())
            .run(cleanup_players(ctx));

        loop {
            scheduler.run_pending();
            thread::sleep(Duration::from_millis(100));
        }
    });
}
