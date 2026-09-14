use std::time::Duration;

use crate::ROBOT;

pub fn spawn_odometry_thread() {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(100));
        let mut last_update = std::time::Instant::now();

        loop {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_update);
            if dt < Duration::from_millis(10) {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }
            last_update = now;

            ROBOT.odometry_state_mut().update(dt);
        }
    });
}