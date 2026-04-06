//! Hardware and application constants for WashinOC.
//!
//! All timing values are in milliseconds unless otherwise noted.
//! All duty-cycle values are in percent (0–100).

/// Touch button debounce window.
pub const DEBOUNCE_MS: u64 = 50;

/// Hold duration to register a long press.
pub const LONG_PRESS_MS: u64 = 800;

/// Idle timeout before entering deep sleep (after first tap, before long-press start).
pub const IDLE_TIMEOUT_MS: u64 = 10_000;

/// Motor forward phase duration.
pub const MOTOR_FORWARD_MS: u64 = 10_000;

/// Motor stop gap between direction changes.
pub const MOTOR_STOP_MS: u64 = 2_000;

/// Motor reverse phase duration.
pub const MOTOR_REVERSE_MS: u64 = 10_000;

/// PWM frequency for motor driver (Hz).
pub const MOTOR_PWM_HZ: u32 = 1_000;

/// Duty cycle for LO (weak) mode — approx 6 V from 9 V supply.
pub const DUTY_LO: u8 = 67;

/// Duty cycle for HI (strong) mode — approx 8 V from 9 V supply.
pub const DUTY_HI: u8 = 89;

/// Safety delay before reversing motor direction.
/// The driver module datasheet requires ≥ 500 ms.
pub const MOTOR_DIRECTION_CHANGE_GUARD_MS: u64 = MOTOR_STOP_MS;
