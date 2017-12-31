extern crate hidapi;
use hidapi::HidApi;

use std::io::Write;

const REPORT_LEN: usize = 64;

pub fn run() -> Result<(), String> {
    let mgr = HidApi::new().unwrap();
    let devices = mgr.devices();

    let vid = 0x054c;
    let pid = 0x05c4;

    let _info = devices.into_iter().find(|info| info.vendor_id == vid && info.product_id == pid)
        .ok_or_else(|| format!("Failed to find a HID with a Vendor ID of {:#X} and a Product ID of {:#X}. Is your DS4 plugged in?", vid, pid))?;

    let dev = mgr.open(vid, pid).map_err(|_msg|
        String::from("Failed to open the DS4 device. Have you setup `udev` rules? (if not, temporarily run `sudo ./target/debug/main`)"))?;

    let mut buf = [0u8; REPORT_LEN];

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let mut report: Report;

    loop {
        let res = dev.read(&mut buf[..]);
        debug_assert_eq!(Ok(REPORT_LEN), res);

        report = Report::from_bytes(buf);

        lock.write(format!("Square: {}\n",   report.is_button_pressed(Button::Square))  .as_bytes()).expect("Writing to stdout failed");
        lock.write(format!("Cross: {}\n",    report.is_button_pressed(Button::Cross))   .as_bytes()).expect("Writing to stdout failed");
        lock.write(format!("Circle: {}\n",   report.is_button_pressed(Button::Circle))  .as_bytes()).expect("Writing to stdout failed");
        lock.write(format!("Triangle: {}\n", report.is_button_pressed(Button::Triangle)).as_bytes()).expect("Writing to stdout failed");
        lock.write(b"\n")                                                                           .expect("Writing to stdout failed");
        lock.flush()                                                                                .expect("Writing to stdout failed");
    }
}

struct Report {
    data: Vec<u8>,
}

impl Report {
    pub fn from_bytes(data: [u8; 64]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    pub fn is_button_pressed(&self, button: Button) -> bool {
        debug_assert_eq!(REPORT_LEN, self.data.len());

        let nybble = self.data[5] >> 4;
        match button {
            Button::Square   => nybble & 1 != 0,
            Button::Cross    => nybble & 2 != 0,
            Button::Circle   => nybble & 4 != 0,
            Button::Triangle => nybble & 8 != 0,
        }
    }

    pub fn is_dpad_pressed(&self, dpad: DPad) -> bool {
        debug_assert_eq!(REPORT_LEN, self.data.len());

        let nybble = self.data[5] & 0x0f;
        match dpad {
            // NOTE: 0x8 represents "no dpad pressed".
            DPad::Up        => nybble == 0b0000,
            DPad::UpRight   => nybble == 0b0001,
            DPad::UpLeft    => nybble == 0b0111,
            DPad::Right     => nybble == 0b0010,
            DPad::Down      => nybble == 0b0100,
            DPad::DownRight => nybble == 0b0011,
            DPad::DownLeft  => nybble == 0b0101,
            DPad::Left      => nybble == 0b0110,
        }
    }
}

enum Button {
    Square,
    Cross,
    Circle,
    Triangle,
}

enum DPad {
    Up,
    UpRight,
    UpLeft,
    Right,
    Down,
    DownRight,
    DownLeft,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_button_pressed {
        ($button:tt, $data:expr) => {
            let mut report = Report { data: vec![0u8; REPORT_LEN] };
            report.data[5] = $data;

            let res = report.is_button_pressed(Button::$button);
            let expected = true;
            assert_eq!(expected, res);
        };
    }

    macro_rules! assert_dpad_pressed {
        ($dpad:tt, $data:expr) => {
            let mut report = Report { data: vec![0u8; REPORT_LEN] };
            report.data[5] = $data;

            let res = report.is_dpad_pressed(DPad::$dpad);
            let expected = true;
            assert_eq!(expected, res);
        };
    }

    #[test]
    fn button_pressed() {
        assert_button_pressed!(Square, 0b00010000);
        assert_button_pressed!(Cross, 0b00100000);
        assert_button_pressed!(Circle, 0b01000000);
        assert_button_pressed!(Triangle, 0b10000000);
    }

    #[test]
    fn dpad_pressed() {
        assert_dpad_pressed!(Up, 0b00000000);
        assert_dpad_pressed!(UpRight, 0b00000001);
        assert_dpad_pressed!(UpLeft, 0b00000111);
        assert_dpad_pressed!(Right, 0b00000010);
        assert_dpad_pressed!(Down, 0b00000100);
        assert_dpad_pressed!(DownRight, 0b00000011);
        assert_dpad_pressed!(DownLeft, 0b00000101);
        assert_dpad_pressed!(Left, 0b00000110);
    }
}