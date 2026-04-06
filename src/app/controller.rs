use esp_hal::time::Instant;

use crate::app::state::RunState;
use crate::app::timing::SoftTimer;
use crate::config;
use crate::domain::mode::WashMode;
use crate::domain::phase::MotorPhase;
use crate::domain::time::RemainingTime;
use crate::drivers::display::Display;
use crate::drivers::motor::{Motor, MotorDirection};
use crate::drivers::touch::{ButtonEvent, TouchButton};

pub fn main_loop(
    touch: &mut TouchButton<'_>,
    display: &mut Display<'_>,
    motor: &mut Motor<'_>,
) -> ! {
    let mut state = RunState::Sleeping;
    let mut idle_timer = SoftTimer::new();
    let mut phase_timer = SoftTimer::new();
    let mut program_timer = SoftTimer::new();

    display.clear();
    motor.set(MotorDirection::Stop, 0);

    loop {
        let now = Instant::now();
        let event = touch.poll(now);

        state = match state {
            RunState::Sleeping => handle_sleeping(event, display, &mut idle_timer, now),

            RunState::Selecting { mode } => handle_selecting(
                event,
                mode,
                display,
                motor,
                &mut idle_timer,
                &mut phase_timer,
                &mut program_timer,
                now,
            ),

            RunState::Running { mode, phase, .. } => handle_running(
                event,
                mode,
                phase,
                display,
                motor,
                &mut phase_timer,
                &mut program_timer,
                now,
            ),

            RunState::Paused {
                mode,
                remaining_ms,
                phase,
                phase_remaining_ms,
            } => handle_paused(
                event,
                mode,
                remaining_ms,
                phase,
                phase_remaining_ms,
                display,
                motor,
                &mut phase_timer,
                &mut program_timer,
                now,
            ),

            RunState::Finishing => handle_finishing(display, motor),
        };
    }
}

fn handle_sleeping(
    event: Option<ButtonEvent>,
    display: &mut Display<'_>,
    idle_timer: &mut SoftTimer,
    now: Instant,
) -> RunState {
    match event {
        Some(ButtonEvent::ShortPress) => {
            let mode = WashMode::Min5Lo;
            display.show_mode_label(mode.label());
            idle_timer.start(now, config::IDLE_TIMEOUT_MS);
            RunState::Selecting { mode }
        }
        _ => RunState::Sleeping,
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_selecting(
    event: Option<ButtonEvent>,
    mode: WashMode,
    display: &mut Display<'_>,
    motor: &mut Motor<'_>,
    idle_timer: &mut SoftTimer,
    phase_timer: &mut SoftTimer,
    program_timer: &mut SoftTimer,
    now: Instant,
) -> RunState {
    if idle_timer.is_expired(now) {
        display.clear();
        return RunState::Sleeping;
    }

    match event {
        Some(ButtonEvent::ShortPress) => {
            let next = mode.next();
            display.show_mode_label(next.label());
            idle_timer.start(now, config::IDLE_TIMEOUT_MS);
            RunState::Selecting { mode: next }
        }
        Some(ButtonEvent::LongPress) => {
            idle_timer.cancel();
            start_program(mode, display, motor, phase_timer, program_timer, now)
        }
        _ => RunState::Selecting { mode },
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_running(
    event: Option<ButtonEvent>,
    mode: WashMode,
    phase: MotorPhase,
    display: &mut Display<'_>,
    motor: &mut Motor<'_>,
    phase_timer: &mut SoftTimer,
    program_timer: &mut SoftTimer,
    now: Instant,
) -> RunState {
    if program_timer.is_expired(now) {
        return finish(motor, phase_timer, program_timer);
    }

    let prog_remaining = program_timer.remaining_ms(now);
    let time = RemainingTime::from_ms(prog_remaining);
    display.show_time(time.minutes, time.seconds);

    if phase_timer.is_expired(now) {
        let next_phase = phase.next();
        apply_motor_phase(next_phase, mode.duty(), motor);
        phase_timer.start(now, next_phase.duration_ms());
        return RunState::Running {
            mode,
            remaining_ms: prog_remaining,
            phase: next_phase,
            phase_remaining_ms: next_phase.duration_ms(),
        };
    }

    match event {
        Some(ButtonEvent::ShortPress) => {
            motor.set(MotorDirection::Stop, 0);
            let pr = phase_timer.remaining_ms(now);
            phase_timer.cancel();
            program_timer.cancel();
            RunState::Paused {
                mode,
                remaining_ms: prog_remaining,
                phase,
                phase_remaining_ms: pr,
            }
        }
        Some(ButtonEvent::LongPress) => finish(motor, phase_timer, program_timer),
        _ => RunState::Running {
            mode,
            remaining_ms: prog_remaining,
            phase,
            phase_remaining_ms: phase_timer.remaining_ms(now),
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_paused(
    event: Option<ButtonEvent>,
    mode: WashMode,
    remaining_ms: u64,
    phase: MotorPhase,
    phase_remaining_ms: u64,
    _display: &mut Display<'_>,
    motor: &mut Motor<'_>,
    phase_timer: &mut SoftTimer,
    program_timer: &mut SoftTimer,
    now: Instant,
) -> RunState {
    match event {
        Some(ButtonEvent::ShortPress) => {
            program_timer.start(now, remaining_ms);
            phase_timer.start(now, phase_remaining_ms);
            apply_motor_phase(phase, mode.duty(), motor);
            RunState::Running {
                mode,
                remaining_ms,
                phase,
                phase_remaining_ms,
            }
        }
        Some(ButtonEvent::LongPress) => finish(motor, phase_timer, program_timer),
        _ => RunState::Paused {
            mode,
            remaining_ms,
            phase,
            phase_remaining_ms,
        },
    }
}

fn handle_finishing(display: &mut Display<'_>, motor: &mut Motor<'_>) -> RunState {
    motor.set(MotorDirection::Stop, 0);
    display.clear();
    RunState::Sleeping
}

fn start_program(
    mode: WashMode,
    display: &mut Display<'_>,
    motor: &mut Motor<'_>,
    phase_timer: &mut SoftTimer,
    program_timer: &mut SoftTimer,
    now: Instant,
) -> RunState {
    let phase = MotorPhase::Forward;
    program_timer.start(now, mode.duration_ms());
    phase_timer.start(now, phase.duration_ms());
    apply_motor_phase(phase, mode.duty(), motor);

    let time = RemainingTime::from_ms(mode.duration_ms());
    display.show_time(time.minutes, time.seconds);

    RunState::Running {
        mode,
        remaining_ms: mode.duration_ms(),
        phase,
        phase_remaining_ms: phase.duration_ms(),
    }
}

fn apply_motor_phase(phase: MotorPhase, duty: u8, motor: &mut Motor<'_>) {
    match phase {
        MotorPhase::Forward => motor.set(MotorDirection::Forward, duty),
        MotorPhase::Reverse => motor.set(MotorDirection::Reverse, duty),
        MotorPhase::StopAfterForward | MotorPhase::StopAfterReverse => {
            motor.set(MotorDirection::Stop, 0);
        }
    }
}

fn finish(
    motor: &mut Motor<'_>,
    phase_timer: &mut SoftTimer,
    program_timer: &mut SoftTimer,
) -> RunState {
    motor.set(MotorDirection::Stop, 0);
    phase_timer.cancel();
    program_timer.cancel();
    RunState::Finishing
}
