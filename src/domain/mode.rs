/// Wash program mode: duration × strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WashMode {
    /// 5 minutes, weak (≈6 V).
    Min5Lo,
    /// 5 minutes, strong (≈8 V).
    Min5Hi,
    /// 10 minutes, weak (≈6 V).
    Min10Lo,
    /// 10 minutes, strong (≈8 V).
    Min10Hi,
}

impl WashMode {
    /// Total wash duration in milliseconds.
    pub const fn duration_ms(self) -> u64 {
        match self {
            Self::Min5Lo | Self::Min5Hi => 5 * 60 * 1_000,
            Self::Min10Lo | Self::Min10Hi => 10 * 60 * 1_000,
        }
    }

    /// PWM duty cycle (0–100).
    pub const fn duty(self) -> u8 {
        match self {
            Self::Min5Lo | Self::Min10Lo => crate::config::DUTY_LO,
            Self::Min5Hi | Self::Min10Hi => crate::config::DUTY_HI,
        }
    }

    /// Cycle to the next mode (wraps around).
    pub const fn next(self) -> Self {
        match self {
            Self::Min5Lo => Self::Min5Hi,
            Self::Min5Hi => Self::Min10Lo,
            Self::Min10Lo => Self::Min10Hi,
            Self::Min10Hi => Self::Min5Lo,
        }
    }

    /// 4-char label for the TM1637 display during mode selection.
    pub const fn label(self) -> &'static [u8; 4] {
        match self {
            Self::Min5Lo => b"05Lo",
            Self::Min5Hi => b"05Hi",
            Self::Min10Lo => b"10Lo",
            Self::Min10Hi => b"10Hi",
        }
    }
}
