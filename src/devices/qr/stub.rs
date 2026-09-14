use crate::devices::maixcam::circle::MaixcamCircleColor;
use std::{fmt::Display, thread, time::Duration};

#[derive(Debug)]
pub enum DriverHIDDevice {
    Disconnected(String),
}

#[derive(Debug)]
pub struct QrDriver {
    pub device: DriverHIDDevice,
}

impl QrDriver {
    pub fn new() -> Self {
        QrDriver {
            device: DriverHIDDevice::Disconnected("QR not supported on this OS".to_string()),
        }
    }

    pub fn reconnect(&mut self) {
        // still disconnected, life is pain
    }

    pub fn is_connected(&self) -> bool {
        false
    }

    pub fn try_read(&mut self) -> Result<Option<String>, String> {
        thread::sleep(Duration::from_millis(100));
        match &mut self.device {
            DriverHIDDevice::Disconnected(msg) => Err(msg.clone())
        }
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