use crate::api::utils::time_operations::get_timezone_with_default;
use chrono_tz::Tz;
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(tz: &Tz, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(tz.name())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Tz, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let tz_ref = get_timezone_with_default(&s);
    Ok(*tz_ref)
}
