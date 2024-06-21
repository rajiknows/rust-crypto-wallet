use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time_instance() -> u64 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    // duration.as_secs() * 1000000000 + duration.subsec_nanos()

    duration.as_secs() << 30 | duration.subsec_nanos() as u64
}
