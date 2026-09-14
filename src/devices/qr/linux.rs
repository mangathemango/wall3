use crate::devices::maixcam::circle::MaixcamCircleColor;
use std::{thread, time::Duration};
use std::sync::Arc;
use std::fmt::Display;
use evdev::{Device, EventSummary, KeyCode};
const QR_READER_DOTENV_KEY: &str = "QR_READER_PATH";

#[derive(Debug)]
pub enum DriverHIDDevice {
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
    pub device: DriverHIDDevice,
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

#[derive(Debug, Clone, Default)]
pub struct QrState {
    pub driver_is_connected: bool,
    pub code: Option<String>,
    pub color_queue_1: Option<Vec<MaixcamCircleColor>>,
    pub color_queue_2: Option<Vec<MaixcamCircleColor>>,
    pub error_msg: String,
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl QrState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, code: String) {
        self.code = Some(code.clone());
        if let Some((left, right)) = code.split_once('+') {
            self.color_queue_1 = Self::parse_code(left.to_string());
            self.color_queue_2 = Self::parse_code(right.to_string());
        } else {
            self.color_queue_1 = None;
            self.color_queue_2 = None;
        }
    }

    pub fn parse_code(code: String) -> Option<Vec<MaixcamCircleColor>> {
        if code.len() != 3 || code.chars().any(|a| !"123".contains(a)){
            return None;
        }
        let result: Vec<MaixcamCircleColor> = code.chars().map(|c|
            match c {
                '1' => MaixcamCircleColor::Red,
                '2' => MaixcamCircleColor::Green,
                '3' => MaixcamCircleColor::Blue,
                _ => Default::default()
            }
        ).collect();
        Some(result)
    }
    
    pub fn reset(&mut self) {
        self.code = None;
        self.color_queue_1 = None;
        self.color_queue_2 = None;
    }
}

impl Display for QrState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connected: {}\nCode:{:?}\nColor Queue 1:{:?}\nColor Queue 2:{:?}",
                self.driver_is_connected, self.code, self.color_queue_1, self.color_queue_2)
    }
}