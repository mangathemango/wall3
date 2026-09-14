use std::fmt::Display;

use crate::{ROBOT, dashboard::helpers::format_radian, devices::gyro::sample::GyroSample};

/// Current gyroscope state
#[derive(Debug, Default, Clone)]
pub struct GyroState {
    /// flag to indicate activity
    pub driver_is_connected: bool,
    pub error_msg: Option<String>,

    /// Current yaw recorded from gyro
    pub yaw: f32,
    /// y angular acceleration
    pub gy: f32,
    /// z angular acceleration
    pub gz: f32,
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl GyroState {
    pub fn new() -> Self {
        GyroState {
            driver_is_connected: false,
            ..Default::default()
        }
    }

    pub fn update(&mut self, sample: GyroSample) {
        self.yaw = sample.yaw;
        self.gy = sample.gy;
        self.gz = sample.gz;
    }

    pub fn publish(&self) {
        ROBOT.set_gyro_state(self.clone());
    }
}

impl Display for GyroState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Raw yaw: {}\nGY: {:.2}\nGZ: {:.2}\nConnected: {}\ndt: {:?}\nError: {:?}",
            format_radian(self.yaw),
            self.gy,
            self.gz,
            self.driver_is_connected,
            self.dt,
            self.error_msg
        )
    }
}
