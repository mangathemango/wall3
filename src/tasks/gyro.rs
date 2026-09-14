use std::time::Duration;

use crate::devices::gyro::{driver::GyroDriver, state::GyroState};

pub fn spawn_gyro_thread() {
    std::thread::spawn(move || {
        let mut driver = GyroDriver::new();
        let mut state = GyroState::new();
        let mut last_update = std::time::Instant::now();

        loop {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_update);
            if dt < Duration::from_millis(20) {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }
            state.dt = now.duration_since(last_update);
            last_update = now;

            match driver.try_read_frame() {
                Ok(sample) => {
                    state.error_msg = None;
                    state.update(sample);
                }
                Err(msg) => {
                    state.error_msg = Some(msg);
                    driver.reconnect();
                }
            };
            state.driver_is_connected = driver.is_connected();
            if !driver.is_connected() {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            state.publish();
        }
    });
}
