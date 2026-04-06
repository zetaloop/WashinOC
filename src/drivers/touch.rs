use esp_hal::gpio::Input;
use esp_hal::time::Instant;

use crate::config;

pub enum ButtonEvent {
    ShortPress,
    LongPress,
}

pub struct TouchButton<'d> {
    pin: Input<'d>,
    pressed_at: Option<Instant>,
    was_pressed: bool,
    long_fired: bool,
}

impl<'d> TouchButton<'d> {
    pub fn new(pin: Input<'d>) -> Self {
        Self {
            pin,
            pressed_at: None,
            was_pressed: false,
            long_fired: false,
        }
    }

    pub fn poll(&mut self, now: Instant) -> Option<ButtonEvent> {
        let is_high = self.pin.is_high();

        if is_high && !self.was_pressed {
            self.pressed_at = Some(now);
            self.was_pressed = true;
            self.long_fired = false;
            return None;
        }

        if is_high
            && self.was_pressed
            && !self.long_fired
            && let Some(start) = self.pressed_at
        {
            let held = now - start;
            if held >= esp_hal::time::Duration::from_millis(config::LONG_PRESS_MS) {
                self.long_fired = true;
                return Some(ButtonEvent::LongPress);
            }
        }

        if !is_high && self.was_pressed {
            self.was_pressed = false;
            let event = if self.long_fired {
                None
            } else if let Some(start) = self.pressed_at {
                let held = now - start;
                if held >= esp_hal::time::Duration::from_millis(config::DEBOUNCE_MS) {
                    Some(ButtonEvent::ShortPress)
                } else {
                    None
                }
            } else {
                None
            };
            self.pressed_at = None;
            return event;
        }

        None
    }

    pub fn is_pressed(&self) -> bool {
        self.pin.is_high()
    }
}
