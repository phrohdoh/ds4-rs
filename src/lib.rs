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

        lock.write(format!("Square: {}\n",   report.is_button_down(Button::Square))  .as_bytes()).expect("Writing to stdout failed");
        lock.write(format!("Cross: {}\n",    report.is_button_down(Button::Cross))   .as_bytes()).expect("Writing to stdout failed");
        lock.write(format!("Circle: {}\n",   report.is_button_down(Button::Circle))  .as_bytes()).expect("Writing to stdout failed");
        lock.write(format!("Triangle: {}\n", report.is_button_down(Button::Triangle)).as_bytes()).expect("Writing to stdout failed");
        lock.write(b"\n")                                                                        .expect("Writing to stdout failed");
        lock.flush()                                                                             .expect("Writing to stdout failed");
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

    pub fn is_button_down(&self, button: Button) -> bool {
        debug_assert_eq!(REPORT_LEN, self.data.len());

        let nybble = self.data[5] >> 4;
        match button {
            Button::Square   => nybble & 1 != 0,
            Button::Cross    => nybble & 2 != 0,
            Button::Circle   => nybble & 4 != 0,
            Button::Triangle => nybble & 8 != 0,
        }
    }
}

enum Button {
    Square,
    Cross,
    Circle,
    Triangle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btn_down_square() {
        let mut report = Report { data: vec![0u8; REPORT_LEN] };
        report.data[5] = 0b00010000;

        let res = report.is_button_down(Button::Square);
        let expected = true;
        assert_eq!(expected, res);
    }

    #[test]
    fn btn_down_cross() {
        let mut report = Report { data: vec![0u8; REPORT_LEN] };
        report.data[5] = 0b00100000;

        let res = report.is_button_down(Button::Cross);
        let expected = true;
        assert_eq!(expected, res);
    }

    #[test]
    fn btn_down_circle() {
        let mut report = Report { data: vec![0u8; REPORT_LEN] };
        report.data[5] = 0b01000000;

        let res = report.is_button_down(Button::Circle);
        let expected = true;
        assert_eq!(expected, res);
    }

    #[test]
    fn btn_down_triangle() {
        let mut report = Report { data: vec![0u8; REPORT_LEN] };
        report.data[5] = 0b10000000;

        let res = report.is_button_down(Button::Triangle);
        let expected = true;
        assert_eq!(expected, res);
    }
}