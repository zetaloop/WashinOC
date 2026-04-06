use esp_hal::gpio::{DriveMode, Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::ledc::channel::{self, ChannelIFace, Number as ChannelNumber};
use esp_hal::ledc::timer::{self, TimerIFace};
use esp_hal::ledc::{LSGlobalClkSource, Ledc, LowSpeed};
use esp_hal::peripherals::Peripherals;
use esp_hal::rtc_cntl::sleep::{Ext0WakeupSource, WakeupLevel};
use esp_hal::rtc_cntl::Rtc;
use esp_hal::time::Rate;

use crate::config;
use crate::drivers::display::Display;
use crate::drivers::motor::Motor;
use crate::drivers::touch::TouchButton;

pub fn run(mut p: Peripherals) -> ! {
    // Touch sensor: GPIO4, active HIGH, pull down (reborrow so we can reclaim for deep sleep)
    let touch_pin = Input::new(
        p.GPIO4.reborrow(),
        InputConfig::default().with_pull(Pull::Down),
    );
    let mut touch = TouchButton::new(touch_pin);

    // TM1637: CLK=GPIO16 (push-pull), DIO=GPIO17 (open-drain + pull-up for ACK)
    let clk = Output::new(p.GPIO16, Level::Low, OutputConfig::default());
    let dio = Output::new(
        p.GPIO17,
        Level::High,
        OutputConfig::default()
            .with_drive_mode(DriveMode::OpenDrain)
            .with_pull(Pull::Up),
    );
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

    crate::app::controller::main_loop(&mut touch, &mut display, &mut motor);

    // main_loop returned — time to enter deep sleep
    // drop touch to end GPIO4 reborrow, allowing move into Ext0WakeupSource
    #[allow(clippy::drop_non_drop)]
    core::mem::drop(touch);

    let mut rtc = Rtc::new(p.LPWR);
    let ext0 = Ext0WakeupSource::new(p.GPIO4, WakeupLevel::High);
    rtc.sleep_deep(&[&ext0])
}
