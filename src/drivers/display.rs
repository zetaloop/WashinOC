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
            WashMode::Min5Lo => [SEG_BLANK, DIGITS[5] | SEG_COLON, SEG_L, SEG_O],
            WashMode::Min5Hi => [SEG_BLANK, DIGITS[5] | SEG_COLON, SEG_H, SEG_I],
            WashMode::Min10Lo => [DIGITS[1], DIGITS[0] | SEG_COLON, SEG_L, SEG_O],
            WashMode::Min10Hi => [DIGITS[1], DIGITS[0] | SEG_COLON, SEG_H, SEG_I],
        };
        let _ = self.tm.display_slice(0, &segments);
    }

    pub fn show_time(&mut self, minutes: u8, seconds: u8) {
        let segments = [
            DIGITS[(minutes / 10) as usize],
            DIGITS[(minutes % 10) as usize] | SEG_COLON,
            DIGITS[(seconds / 10) as usize],
            DIGITS[(seconds % 10) as usize],
        ];
        let _ = self.tm.display_slice(0, &segments);
    }

    pub fn clear(&mut self) {
        let _ = self.tm.clear();
    }
}

const SEG_COLON: u8 = 0x80;
const SEG_BLANK: u8 = 0x00;
const SEG_L: u8 = 0x38;
const SEG_O: u8 = 0x5C;
const SEG_H: u8 = 0x76;
const SEG_I: u8 = 0x10;

#[rustfmt::skip]
const DIGITS: [u8; 10] = [
    0x3F, // 0
    0x06, // 1
    0x5B, // 2
    0x4F, // 3
    0x66, // 4
    0x6D, // 5
    0x7D, // 6
    0x07, // 7
    0x7F, // 8
    0x6F, // 9
];
