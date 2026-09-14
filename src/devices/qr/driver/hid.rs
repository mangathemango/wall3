use std::{
    thread,
    time::Duration,
};

use evdev::{
    Device,
    EventSummary,
    KeyCode,
};
use crate::devices::qr::QR_READER_DOTENV_KEY;   

#[derive(Debug)]
enum DriverHIDDevice {
    Connected(Device),
    Disconnected(String),
}

impl DriverHIDDevice {
    pub fn is_connected(&self) -> bool {
        match self {
            DriverHIDDevice::Connected(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct QrDriver {
    device: DriverHIDDevice,
}

impl QrDriver {
    pub fn new() -> Self {
        let path = match dotenv::var(QR_READER_DOTENV_KEY) {
            Ok(path) => path,
            Err(e) => {
                return QrDriver {
                    device: DriverHIDDevice::Disconnected(format!("Env error: {}", e).into()),
                };
            }
        };
        let device = match Device::open(path) {
            Ok(device) => device,
            Err(e) => {
                return QrDriver {
                    device: DriverHIDDevice::Disconnected(
                        format!("Open Qrcode device error: {}", e).into(),
                    ),
                };
            }
        };

        QrDriver {
            device: DriverHIDDevice::Connected(device),
        }
    }
    pub fn reconnect(&mut self) {
        let path = match dotenv::var(QR_READER_DOTENV_KEY) {
            Ok(path) => path,
            Err(e) => {
                self.device = DriverHIDDevice::Disconnected(format!("Env error: {}", e));
                return;
            }
        };

        match Device::open(path) {
            Ok(device) => {
                self.device = DriverHIDDevice::Connected(device);
            }
            Err(e) => {
                self.device = DriverHIDDevice::Disconnected(format!("Open error: {}", e));
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_connected()
    }

    pub fn try_read(&mut self) -> Result<Option<String>, String> {
        let mut buffer = String::new();

        for _ in 0..100 {
            let evs = match &mut self.device {
                DriverHIDDevice::Connected(device) => match device.fetch_events() {
                    Ok(evs) => evs,
                    Err(e) => {
                        return Err(format!("Fetch event from Qr code error: {}", e));
                    }
                },
                DriverHIDDevice::Disconnected(msg) => {
                    return Err(msg.clone());
                }
            };

            for ev in evs {
                if let EventSummary::Key(_, keycode, value) = ev.destructure() {
                    if value == 1 {
                        if keycode == KeyCode::KEY_ENTER {
                            return Ok(Some(buffer));
                        }

                        if let Some(c) = Self::keycode_to_ascii(keycode) {
                            buffer.push(c);
                        }
                    }
                }
            }

            thread::sleep(Duration::from_millis(10));
        }

        Ok(None)
    }

    pub fn keycode_to_ascii(key: KeyCode) -> Option<char> {
        let c = match key {
            KeyCode::KEY_1 => '1',
            KeyCode::KEY_2 => '2',
            KeyCode::KEY_3 => '3',
            KeyCode::KEY_4 => '4',
            KeyCode::KEY_5 => '5',
            KeyCode::KEY_6 => '6',
            KeyCode::KEY_7 => '7',
            KeyCode::KEY_8 => '8',
            KeyCode::KEY_9 => '9',
            KeyCode::KEY_0 => '0',

            // The Qr reader inputs a "+" by doing KeyCode::Shift THEN a Keycode::Equal.
            // It's alr I'm just gonna hard code it
            KeyCode::KEY_EQUAL => '+',

            _ => return None,
        };

        // optional shift handling
        Some(c)
    }
}