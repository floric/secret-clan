mod housekeeping;

use clokwerk::{Scheduler, TimeUnits};
use std::{thread, time::Duration};

use self::housekeeping::{cleanup_games, cleanup_players};

const JOB_INTERVAL: u32 = 60;

pub fn init_jobs() {
    tokio::spawn(async {
        let mut scheduler = Scheduler::new();

        scheduler.every(JOB_INTERVAL.seconds()).run(cleanup_games);
        scheduler.every(JOB_INTERVAL.seconds()).run(cleanup_players);

        loop {
            scheduler.run_pending();
            thread::sleep(Duration::from_millis(100));
        }
    });
}
