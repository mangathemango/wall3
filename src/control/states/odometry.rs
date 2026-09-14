use glam::Vec2;

use crate::ROBOT;
use crate::math::{MecanumVelocities, Pose, Twist, utils::wrap_angle};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct OdometryState {
    pub twist: Twist,
    pub current_pose: Pose,
    pub pursuit_pose: Pose,
    pub gyro_offset: f32,
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl OdometryState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, dt: Duration) {
        self.dt = dt;

        let stm32_state = ROBOT.stm32_state();
        let gyro_state = ROBOT.gyro_state();

        if self.gyro_offset.is_nan() {
            self.gyro_offset = gyro_state.yaw;
        }

        let [vfl, vfr, vrl, vrr] = stm32_state
            .actual_wheel_velocities
            .map(|v| v as f32 / 10000.0);

        self.twist = MecanumVelocities::new(vfl, vfr, vrl, vrr).to_twist();

        let translation =
            (self.twist.linear * dt.as_secs_f32()).rotate(Vec2::from_angle(-self.current_pose.rotation));

        self.current_pose.position += translation;
        self.current_pose.rotation = wrap_angle(gyro_state.yaw - self.gyro_offset);
    }

    pub fn set_current_rotation(&mut self, rotation: f32) {
        let gyro_state = ROBOT.gyro_state();
        self.gyro_offset = wrap_angle(gyro_state.yaw - rotation);
    }

    pub fn add_gyro_offset(&mut self, offset: f32) {
        self.gyro_offset = wrap_angle(self.gyro_offset + offset);
    }


    pub fn set_current_pose(&mut self, pose: Pose) {
        self.current_pose = pose;
    }
}
