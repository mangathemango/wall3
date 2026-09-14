use std::time::{Duration, Instant};

use crate::devices::maixcam::{driver::MaixcamDriver, message::MaixcamMessage, state::MaixcamState};

/// The Maixcam does nothing but send circle coordinates, so it doesn't really need to scale for the time being
/// A packet of data is formatted like this:
/// [START] [ID] [LEN] [DATA] [CHECKSUM]
/// Where pos_x/y is a float from 0 to RESOLUTION_WIDTH/HEIGHT mapped into the range of 0..10000
pub fn spawn_maixcam_thread() {
    std::thread::spawn(move || {
        let mut driver = MaixcamDriver::new();
        let mut state = MaixcamState::new();
        let mut last_update = std::time::Instant::now();

        loop {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_update);
            if dt < Duration::from_millis(50) {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }
            state.dt = now.duration_since(last_update);
            last_update = now;

            match driver.try_read_frame() {
                Ok(messages) => {
                    state.error = None;
                    for message in messages {
                        match message {
                            MaixcamMessage::CircleData(circles) => {
                                state.circles = circles
                                    .into_iter()
                                    .map(|current_circle| {
                                        let last_circle =
                                            state.last_circles.iter().find(|last_circle| {
                                                last_circle.color == current_circle.color
                                            });
                                        let mut result = current_circle;
                                        if let Some(last_circle) = last_circle {
                                            result.speed = (current_circle.position - last_circle.position).length() / dt.as_secs_f32();
                                        } else {
                                            result.speed = 0.0;
                                        }
                                        result
                                    })
                                    .collect();
                            }
                        }
                    }
                    state.last_updated = Instant::now();
                }
                Err(msg) => {
                    state.circles.clear();
                    state.driver_is_connected = false;
                    state.error = Some(msg);
                    state.publish();
                    driver.reconnect();
                }
            }
            if Instant::now().duration_since(state.last_updated) > Duration::from_millis(1000) {
                state.last_circles.clear();
                state.circles.clear();
            }
            state.last_circles = state.circles.clone();
            state.driver_is_connected = driver.is_connected();
            state.publish();
        }
    });
}
