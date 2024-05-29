use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct AppUsageData {
    pub last_active: Instant,
    pub time_spent: HashMap<String, (i64, Duration)>, // Mapping app_name to (window_id, Duration)
}

impl Default for AppUsageData {
    fn default() -> Self {
        Self {
            last_active: Instant::now(),
            time_spent: HashMap::new(),
        }
    }
}
