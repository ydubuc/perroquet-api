use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn current_time_in_secs() -> i64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
    let seconds = since_epoch.as_secs() as i64;

    seconds
}

pub fn current_time_in_millis() -> i64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
    let milliseconds = since_epoch.as_secs() as i64 * 1000 + i64::from(since_epoch.subsec_millis());

    milliseconds
}
