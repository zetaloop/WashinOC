use esp_hal::ledc::channel::{self, ChannelIFace};
use esp_hal::ledc::LowSpeed;

pub enum MotorDirection {
    Forward,
    Reverse,
    Stop,
    Brake,
}

pub struct Motor<'d> {
    ch_in1: channel::Channel<'d, LowSpeed>,
    ch_in2: channel::Channel<'d, LowSpeed>,
}

impl<'d> Motor<'d> {
    pub fn new(
        ch_in1: channel::Channel<'d, LowSpeed>,
        ch_in2: channel::Channel<'d, LowSpeed>,
    ) -> Self {
        Self { ch_in1, ch_in2 }
    }

    pub fn set(&mut self, direction: MotorDirection, duty_pct: u8) {
        let (in1_duty, in2_duty) = match direction {
            MotorDirection::Forward => (duty_pct, 0),
            MotorDirection::Reverse => (0, duty_pct),
            MotorDirection::Stop => (0, 0),
            MotorDirection::Brake => (100, 100),
        };

        let _ = self.ch_in1.set_duty(in1_duty);
        let _ = self.ch_in2.set_duty(in2_duty);
    }
}
