use std::time::{SystemTime, UNIX_EPOCH};

/// Get unix timestamp as a 64-bit unsigned integer
pub fn now() -> u64 {
    // Get system time
    let start = SystemTime::now();

    // Return timestamp as an integer
    start.duration_since(UNIX_EPOCH).unwrap().as_secs()
}
