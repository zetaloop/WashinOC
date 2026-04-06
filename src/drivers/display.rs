use esp_hal::gpio::Output;

pub struct Display<'d> {
    clk: Output<'d>,
    dio: Output<'d>,
    brightness: u8,
}

impl<'d> Display<'d> {
    pub fn new(clk: Output<'d>, dio: Output<'d>) -> Self {
        Self {
            clk,
            dio,
            brightness: 0x07,
        }
    }

    pub fn show_mode_label(&mut self, label: &[u8; 4]) {
        let segments = label.map(ascii_to_segment);
        self.write_segments(&segments);
    }

    pub fn show_time(&mut self, minutes: u8, seconds: u8) {
        let segs = [
            DIGITS[(minutes / 10) as usize],
            DIGITS[(minutes % 10) as usize] | SEG_COLON,
            DIGITS[(seconds / 10) as usize],
            DIGITS[(seconds % 10) as usize],
        ];
        self.write_segments(&segs);
    }

    pub fn clear(&mut self) {
        self.write_segments(&[0x00; 4]);
    }

    fn write_segments(&mut self, segments: &[u8; 4]) {
        self.start();
        self.write_byte(0x40);
        self.stop();

        self.start();
        self.write_byte(0xC0);
        for &seg in segments {
            self.write_byte(seg);
        }
        self.stop();

        self.start();
        self.write_byte(0x88 | (self.brightness & 0x07));
        self.stop();
    }

    fn start(&mut self) {
        self.clk.set_high();
        self.dio.set_high();
        self.delay();
        self.dio.set_low();
        self.delay();
        self.clk.set_low();
        self.delay();
    }

    fn stop(&mut self) {
        self.clk.set_low();
        self.dio.set_low();
        self.delay();
        self.clk.set_high();
        self.delay();
        self.dio.set_high();
        self.delay();
    }

    fn write_byte(&mut self, mut byte: u8) {
        for _ in 0..8 {
            self.clk.set_low();
            if byte & 0x01 != 0 {
                self.dio.set_high();
            } else {
                self.dio.set_low();
            }
            self.delay();
            self.clk.set_high();
            self.delay();
            byte >>= 1;
        }
        // ACK bit (we ignore it since DIO is output-only)
        self.clk.set_low();
        self.dio.set_high();
        self.delay();
        self.clk.set_high();
        self.delay();
        self.clk.set_low();
        self.delay();
    }

    #[inline(always)]
    fn delay(&self) {
        // ~2µs at 240MHz ≈ 480 nops
        for _ in 0..100 {
            core::hint::spin_loop();
        }
    }
}

const SEG_COLON: u8 = 0x80;

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

fn ascii_to_segment(ch: u8) -> u8 {
    match ch {
        b'0'..=b'9' => DIGITS[(ch - b'0') as usize],
        b'L' => 0x38,
        b'o' => 0x5C,
        b'H' => 0x76,
        b'i' => 0x04,
        b' ' => 0x00,
        _ => 0x00,
    }
}
