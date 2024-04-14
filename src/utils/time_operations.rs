use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{offset::LocalResult, TimeZone, Utc};

pub fn timestamp_now_nanos() -> u64 {
    let start_time = SystemTime::now();
    let since_unix = start_time.duration_since(UNIX_EPOCH).expect("Somehow the time went backwards...");
    since_unix.as_nanos() as u64
}

pub fn nanos_to_date(nanos: u64) -> String {
    let seconds = (nanos / 1_000_000_000) as i64;
    let nanos_remaining = (nanos % 1_000_000_000) as u32;
    match Utc.timestamp_opt(seconds, nanos_remaining) {
        LocalResult::Single(datetime) => {
            datetime.format("%Y-%m-%d %H:%M:%S.%f UTC").to_string()
        },
        _ => "Invalid timestamp".to_string(),
    }
}