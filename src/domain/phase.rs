/// Motor phase within one wash cycle (forward → stop → reverse → stop).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorPhase {
    Forward,
    StopAfterForward,
    Reverse,
    StopAfterReverse,
}

impl MotorPhase {
    /// Duration of this phase in milliseconds.
    pub const fn duration_ms(self) -> u64 {
        match self {
            Self::Forward => crate::config::MOTOR_FORWARD_MS,
            Self::StopAfterForward => crate::config::MOTOR_STOP_MS,
            Self::Reverse => crate::config::MOTOR_REVERSE_MS,
            Self::StopAfterReverse => crate::config::MOTOR_STOP_MS,
        }
    }

    /// Advance to the next phase (wraps around).
    pub const fn next(self) -> Self {
        match self {
            Self::Forward => Self::StopAfterForward,
            Self::StopAfterForward => Self::Reverse,
            Self::Reverse => Self::StopAfterReverse,
            Self::StopAfterReverse => Self::Forward,
        }
    }
}
