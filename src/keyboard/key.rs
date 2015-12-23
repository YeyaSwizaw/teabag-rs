use ::keyboard::KeyCode;

pub type ScanCode = u8;

#[derive(Copy, Clone, Debug)]
pub struct Key {
    pub scan_code: ScanCode,
    pub key: KeyCode
}

impl Key {
    pub fn new(scan_code: ScanCode, key_code: Option<::glutin::VirtualKeyCode>) -> Key {
        Key {
            scan_code: scan_code,
            key: key_code.map(|code| KeyCode::from_glutin_code(code)).unwrap_or(KeyCode::Other)
        }
    }
}
