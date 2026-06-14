use esp_hal::delay::Delay;
use esp_hal::gpio::Output;
use tm1637_embedded_hal::tokens::Blocking;
use tm1637_embedded_hal::{Brightness, TM1637Builder};

use crate::domain::mode::WashMode;

pub struct Display<'d> {
    tm: tm1637_embedded_hal::TM1637<4, Blocking, Output<'d>, Output<'d>, Delay>,
}

impl<'d> Display<'d> {
    pub fn new(clk: Output<'d>, dio: Output<'d>) -> Self {
        let mut tm = TM1637Builder::new(clk, dio, Delay::new())
            .brightness(Brightness::L7)
            .delay_us(100)
            .build_blocking::<4>();
        let _ = tm.init();
        Self { tm }
    }

    pub fn show_mode(&mut self, mode: WashMode) {
        let segments = match mode {
            WashMode::Min5Lo => [SEG_O, SEG_L | SEG_COLON, DIGITS[5], SEG_BLANK],
            WashMode::Min5Hi => [SEG_I, SEG_H | SEG_COLON, DIGITS[5], SEG_BLANK],
            WashMode::Min10Lo => [SEG_O, SEG_L | SEG_COLON, DIGITS[0], DIGITS[1]],
            WashMode::Min10Hi => [SEG_I, SEG_H | SEG_COLON, DIGITS[0], DIGITS[1]],
        };
        self.write_segments(&segments);
    }

    pub fn show_time(&mut self, minutes: u8, seconds: u8) {
        let segments = [
            DIGITS[(seconds % 10) as usize],
            DIGITS[(seconds / 10) as usize] | SEG_COLON,
            DIGITS[(minutes % 10) as usize],
            DIGITS[(minutes / 10) as usize],
        ];
        self.write_segments(&segments);
    }

    pub fn clear(&mut self) {
        let _ = self.tm.on();
        let _ = self.tm.clear();
    }

    pub fn show_shutdown(&mut self) {
        self.write_segments(&[SEG_DASH; 4]);
    }

    fn write_segments(&mut self, segments: &[u8; 4]) {
        let _ = self.tm.on();
        let _ = self.tm.display_slice(0, segments);
    }
}

const SEG_COLON: u8 = 0x80;
const SEG_BLANK: u8 = 0x00;
const SEG_DASH: u8 = 0x40;
const SEG_L: u8 = 0x07;
const SEG_O: u8 = 0x63;
const SEG_H: u8 = 0x76;
const SEG_I: u8 = 0x02;

#[rustfmt::skip]
const DIGITS: [u8; 10] = [
    0x3F, // 0
    0x30, // 1
    0x5B, // 2
    0x79, // 3
    0x74, // 4
    0x6D, // 5
    0x6F, // 6
    0x38, // 7
    0x7F, // 8
    0x7D, // 9
];
