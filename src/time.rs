use chrono::Utc;

pub fn get_timestamp() -> i64 {
    Utc::now().timestamp_millis()
}

pub fn cps_to_millis(cps: u32) -> i64 {
    return ((1.0 / cps as f64) * 1000.0) as i64;
}
