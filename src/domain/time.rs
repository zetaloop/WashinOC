/// Remaining time, broken into minutes and seconds for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemainingTime {
    pub minutes: u8,
    pub seconds: u8,
}

impl RemainingTime {
    /// Build from remaining milliseconds.
    pub const fn from_ms(ms: u64) -> Self {
        let total_secs = (ms / 1_000) as u32;
        Self {
            minutes: (total_secs / 60) as u8,
            seconds: (total_secs % 60) as u8,
        }
    }

    /// True when the countdown has reached zero.
    pub const fn is_zero(self) -> bool {
        self.minutes == 0 && self.seconds == 0
    }
}
