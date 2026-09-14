use std::{thread, time::Duration};

#[derive(Debug)]
pub enum DriverHIDDevice {
    Disconnected(String),
}

#[derive(Debug)]
pub struct QrDriver {
    device: DriverHIDDevice,
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