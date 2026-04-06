use esp_hal::time::{Duration, Instant};

pub struct SoftTimer {
    deadline: Option<Instant>,
}

impl Default for SoftTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftTimer {
    pub const fn new() -> Self {
        Self { deadline: None }
    }

    pub fn start(&mut self, now: Instant, duration_ms: u64) {
        self.deadline = Some(now + Duration::from_millis(duration_ms));
    }

    pub fn cancel(&mut self) {
        self.deadline = None;
    }

    pub fn is_expired(&self, now: Instant) -> bool {
        match self.deadline {
            Some(d) => now >= d,
            None => false,
        }
    }

    pub fn remaining_ms(&self, now: Instant) -> u64 {
        match self.deadline {
            Some(d) if now < d => {
                let diff = d - now;
                diff.as_millis()
            }
            _ => 0,
        }
    }
}
