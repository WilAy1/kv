use std::time::{SystemTime, UNIX_EPOCH};

pub fn has_passed(expires_at: u64) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();

    now >= expires_at
}