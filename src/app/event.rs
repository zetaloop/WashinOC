use crate::domain::mode::WashMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    ShortPress,
    LongPress,
    IdleTimeout,
    PhaseElapsed,
    ProgramComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayContent {
    ModeLabel(WashMode),
    Countdown { minutes: u8, seconds: u8 },
    Off,
}
