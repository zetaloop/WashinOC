use esp_hal::gpio::{DriveMode, Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::ledc::channel::{self, ChannelIFace, Number as ChannelNumber};
use esp_hal::ledc::timer::{self, TimerIFace};
use esp_hal::ledc::{LSGlobalClkSource, Ledc, LowSpeed};
use esp_hal::peripherals::Peripherals;
use esp_hal::time::Rate;

use crate::config;
use crate::drivers::display::Display;
use crate::drivers::motor::Motor;
use crate::drivers::touch::TouchButton;

pub fn run(p: Peripherals) -> ! {
    // Touch sensor: GPIO4, active HIGH, pull down
    let touch_pin = Input::new(p.GPIO4, InputConfig::default().with_pull(Pull::Down));
    let mut touch = TouchButton::new(touch_pin);

    // TM1637: CLK=GPIO16, DIO=GPIO17
    let clk = Output::new(p.GPIO16, Level::Low, OutputConfig::default());
    let dio = Output::new(p.GPIO17, Level::Low, OutputConfig::default());
    let mut display = Display::new(clk, dio);

    // Motor PWM: IN1=GPIO18 (Ch0), IN2=GPIO19 (Ch1) via LEDC
    let mut ledc = Ledc::new(p.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty10Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_hz(config::MOTOR_PWM_HZ),
        })
        .expect("LEDC timer config failed");

    let mut ch_in1 = channel::Channel::new(ChannelNumber::Channel0, p.GPIO18);
    ch_in1
        .configure(channel::config::Config {
            timer: &lstimer,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })
        .expect("LEDC ch0 config failed");

    let mut ch_in2 = channel::Channel::new(ChannelNumber::Channel1, p.GPIO19);
    ch_in2
        .configure(channel::config::Config {
            timer: &lstimer,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })
        .expect("LEDC ch1 config failed");

    let mut motor = Motor::new(ch_in1, ch_in2);

    crate::app::controller::main_loop(&mut touch, &mut display, &mut motor)
}
