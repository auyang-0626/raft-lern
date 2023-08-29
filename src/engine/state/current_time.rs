use std::sync::atomic::{AtomicU64, Ordering};

use chrono::Utc;

/// 当前时间
pub(crate) static CURRENT_TIME: AtomicU64 = AtomicU64::new(0);

pub(crate) fn update_current_time() {
    CURRENT_TIME.store(Utc::now().timestamp_millis() as u64, Ordering::Release)
}

pub(crate) fn get_current_time() -> u64 {
    CURRENT_TIME.load(Ordering::Acquire)
}