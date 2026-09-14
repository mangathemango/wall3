use std::{f32::consts::FRAC_PI_2, fmt::Display, time::Duration};

use glam::Vec2;

use crate::{
    ROBOT,
    control::actions::{Action, rotate_arm::ArmRotationPreset},
    devices::maixcam::circle::{MaixcamCircle, MaixcamCircleColor},
    math::{PidController, Pose, Twist},
};

#[derive(Debug, Default)]
pub struct CalibratePlacement {
    chosen_circle: Option<MaixcamCircle>,
    linear_pid: PidController,
    angular_pid: PidController,
    initial_rotation: f32,
    angular_offset: f32,
}

impl CalibratePlacement {
    pub fn new() -> Self {
        Self {
            linear_pid: PidController::new(0.008, 0.0001, 0.0001, 0.01, 1.0),
            angular_pid: PidController::new(-0.004, -0.0005, -0.0005, 0.02, 1.0),
            angular_offset: 0.0,
            ..Default::default()
        }
    }
    

    pub fn with_angular_offset(mut self, offset: f32) -> Self {
        self.angular_offset = offset;
        self
    }

    pub fn while_keeping_rotation(mut self, rotation: f32) -> Self {
        self.initial_rotation = rotation;
        self
    }
}

impl Action for CalibratePlacement {
    fn start(&mut self) {
        ROBOT
            .stm32_controller()
            .set_yaw_servo(ArmRotationPreset::Calibration.to_angle());
    }

    fn update(&mut self, dt: Duration) {
        let current_rotation = ROBOT.odometry_state().current_pose.rotation;
        let maixcam_state = ROBOT.maixcam_state();
        self.chosen_circle = maixcam_state.find_priority_ring(&[
            MaixcamCircleColor::Green,
            MaixcamCircleColor::Blue,
            MaixcamCircleColor::Red,
        ]);

        // let angle_cirlce = maixcam_state.find_priority_ring(&[
        //     MaixcamCircleColor::Red,
        //     MaixcamCircleColor::Blue,
        // ]);

        if let Some(circle) = self.chosen_circle {
            let circle_position = match circle.color {
                MaixcamCircleColor::Blue => circle.position + Vec2::new(0.5, 0.0),
                MaixcamCircleColor::Green => circle.position,
                MaixcamCircleColor::Red => circle.position + Vec2::new(-0.5, 0.0),
            };

            // Move the robot linearly so that the circle ends up in the target position while keeping the initial rotation stable
            let current_state = Pose {
                position: circle_position,
                rotation: current_rotation,
            };

            let target_state = Pose {
                position: Vec2::new(0.46562, 0.60417),
                rotation: self.initial_rotation,
            };

            let linear_error;
            let angular_error;
            // if let Some(angle_circle) = angle_cirlce {
            //     (linear_error, _) = current_state.difference(target_state).to_components();
            //     angular_error = match angle_circle.color {
            //         MaixcamCircleColor::Red => self.mode.target_circle_position() - angle_circle.position,
            //         MaixcamCircleColor::Blue => angle_circle.position - self.mode.target_circle_position(), 
            //         MaixcamCircleColor::Green => Vec2::ZERO,
            //     }.y
            // } else {

            (linear_error, angular_error) = current_state.difference(target_state).to_components();
            // }

            let linear_direction = linear_error.normalize_or_zero();
            let linear_correction_speed = self.linear_pid
                .update(linear_error.length(), dt)
                .clamp(0.0, 0.01);
            let mut linear_correction = linear_direction * linear_correction_speed;

            linear_correction = linear_correction.rotate(Vec2::from_angle(-FRAC_PI_2));

            let angular_correction = self.angular_pid.update(angular_error + self.angular_offset, dt);

            let target_twist = Twist::new(linear_correction, angular_correction);
            ROBOT.stm32_controller().set_twist(target_twist);
        } else {
            ROBOT.stm32_controller().set_twist(Twist::ZERO);
        }
    }

    fn stop(&mut self) {
        ROBOT.stm32_controller().set_twist(Twist::ZERO);
    }

    fn is_finished(&self) -> bool {
        self.linear_pid.is_settled_for(Duration::from_millis(1000))
            && self.angular_pid.is_settled_for(Duration::from_millis(100))
    }
}

impl Display for CalibratePlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Calibrating placment with mode {:?}\nLinear PID: {}\n\nAngular PID: {}",
            self.chosen_circle.unwrap_or_default(),
            self.linear_pid,
            self.angular_pid,
        )
    }
}
