use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct AppUsageData {
    pub last_active: Instant,
    pub time_spent: HashMap<String, Duration>,
}

impl Default for AppUsageData {
    fn default() -> Self {
        Self {
            last_active: Instant::now(),
            time_spent: HashMap::new(),
        }
    }
}

pub type SharedAppUsageData = Arc<Mutex<AppUsageData>>;
