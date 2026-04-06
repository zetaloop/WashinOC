use crate::domain::mode::WashMode;
use crate::domain::phase::MotorPhase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    Sleeping,
    Selecting {
        mode: WashMode,
    },
    Running {
        mode: WashMode,
        phase: MotorPhase,
    },
    Paused {
        mode: WashMode,
        remaining_ms: u64,
        phase: MotorPhase,
        phase_remaining_ms: u64,
    },
    Finishing,
}
