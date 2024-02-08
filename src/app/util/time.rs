use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_in_millis() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let milliseconds =
        since_the_epoch.as_secs() as i64 * 1000 + i64::from(since_the_epoch.subsec_millis());

    return milliseconds;
}
