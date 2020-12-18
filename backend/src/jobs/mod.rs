mod housekeeping;

use clokwerk::{Scheduler, TimeUnits};
use std::{thread, time::Duration};

use crate::server::app_context::AppContext;

use self::housekeeping::{cleanup_games, cleanup_players};

const JOB_INTERVAL: u32 = 10;

pub fn init_jobs(ctx: &'static AppContext) {
    tokio::spawn(async move {
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
