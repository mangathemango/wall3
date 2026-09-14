pub mod driver;
pub mod state;
use std::time::Duration;

use driver::QrDriver;

use crate::ROBOT;
const QR_READER_DOTENV_KEY: &str = "QR_READER_PATH";

pub fn spawn_qr_thread() {
    std::thread::spawn(move || {
        let mut driver = QrDriver::new();
        let mut last_update = std::time::Instant::now();
        ROBOT.qr_state_mut().driver_is_connected = driver.is_connected();
        loop {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_update);
            if dt < Duration::from_millis(20) {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }
            ROBOT.qr_state_mut().dt = dt;
            last_update = now;

            match driver.try_read() {
                Ok(Some(code)) => {
                    ROBOT.qr_state_mut().update(code.clone());
                }
                Ok(None) => {}
                Err(msg) => {
                    ROBOT.qr_state_mut().error_msg = msg.clone();
                    driver.reconnect();
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
            ROBOT.qr_state_mut().driver_is_connected = driver.is_connected();
        }
    });
}
