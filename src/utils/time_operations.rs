use chrono::{offset::LocalResult, TimeZone};
use chrono_tz::{Tz, TZ_VARIANTS};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

lazy_static! {
    static ref TIMEZONES: Vec<String> = TZ_VARIANTS
        .iter()
        .map(|&tz| tz.to_string().to_lowercase())
        .collect();
}

lazy_static! {
    static ref TIMEZONE_MAP: HashMap<String, Tz> = TZ_VARIANTS
        .iter()
        .map(|&tz| (tz.to_string().to_lowercase(), tz))
        .collect();
}

pub fn timestamp_now_nanos() -> u64 {
    let start_time = SystemTime::now();
    let since_unix = start_time
        .duration_since(UNIX_EPOCH)
        .expect("Somehow the time went backwards...");
    since_unix.as_nanos() as u64
}

pub fn nanos_to_date(nanos: u64, tz: &Tz) -> String {
    let seconds = (nanos / 1_000_000_000) as i64;
    let nanos_remaining = (nanos % 1_000_000_000) as u32;
    match tz.timestamp_opt(seconds, nanos_remaining) {
        LocalResult::Single(datetime) => datetime.format("%Y-%m-%d %H:%M:%S.%f %Z").to_string(),
        _ => "Invalid timestamp".to_string(),
    }
}

pub fn get_timezone_names() -> Vec<String> {
    TIMEZONES.clone()
}

pub fn get_timezone_from_name(name: &str) -> Option<&Tz> {
    let name = name.to_lowercase();
    TIMEZONE_MAP.get(&name)
}

pub fn get_timezone_with_default(name: &str) -> &Tz {
    let timezone = get_timezone_from_name(name);
    match timezone {
        Some(tz) => tz,
        None => &chrono_tz::UTC,
    }
}
