use std::{
    cmp::Ordering, collections::HashMap, f32::consts::FRAC_PI_2, fmt::Display, time::Duration,
};

use glam::Vec2;

use crate::{
    ROBOT,
    control::{
        actions::{Action, general::Sequence},
        routines::material_handling::pick_up_all_materials_from_ground,
    },
    devices::maixcam::circle::MaixcamCircleColor,
    math::{PidController, Pose, Twist},
};

pub struct CalibrateTemporary {
    linear_pid: PidController,
    angular_pid: PidController,
    color_order: [MaixcamCircleColor; 3],
    initial_rotation: f32,
    angular_offset: f32,

}

impl CalibrateTemporary {
    pub fn new(color_order: [MaixcamCircleColor; 3]) -> Self {
        Self {
            linear_pid: PidController::new(0.008, 0.0001, 0.0001, 0.01, 1.0),
            angular_pid: PidController::new(-0.004, -0.0005, -0.0005, 0.02, 1.0),
            color_order,
            initial_rotation: 0.0,
            angular_offset: 0.0,
        }
    }
}

impl Action for CalibrateTemporary {
    fn update(&mut self, dt: std::time::Duration) {
        let color_order = self.color_order;
        let current_rotation = ROBOT.odometry_state().current_pose.rotation;
        let maixcam_state = ROBOT.maixcam_state();
        let chosen_circle =
            maixcam_state.find_priority_ring(&[color_order[1], color_order[2], color_order[0]]);

        if let Some(circle) = chosen_circle {
            let mut circle_position = circle.position;
            if circle.color == color_order[0] {
                circle_position += Vec2::new(0.5, 0.0);
            } else if circle.color == color_order[2] {
                circle_position -= Vec2::new(0.5, 0.0);
            }

            // Move the robot linearly so that the circle ends up in the target position while keeping the initial rotation stable
            let current_state = Pose {
                position: circle_position,
                rotation: current_rotation,
            };

            let target_state = Pose {
                position: Vec2::new(0.46562, 0.60417),
                rotation: self.initial_rotation,
            };

            let (linear_error, angular_error) =
                current_state.difference(target_state).to_components();

            let linear_direction = linear_error.normalize_or_zero();
            let linear_correction_speed = self
                .linear_pid
                .update(linear_error.length(), dt)
                .clamp(0.0, 0.01);
            let mut linear_correction = linear_direction * linear_correction_speed;

            linear_correction = linear_correction.rotate(Vec2::from_angle(-FRAC_PI_2));

            let angular_correction = self
                .angular_pid
                .update(angular_error + self.angular_offset, dt);

            let target_twist = Twist::new(linear_correction, angular_correction);
            ROBOT.stm32_controller().set_twist(target_twist);
        } else {
            ROBOT.stm32_controller().set_twist(Twist::ZERO);
        }
    }

    fn is_finished(&self) -> bool {
        self.linear_pid.is_settled_for(Duration::from_millis(1000))
            && self.angular_pid.is_settled_for(Duration::from_millis(1000))
    }
}

impl Display for CalibrateTemporary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== CalibrateTemporary ===")?;
        writeln!(f, "Color Order     : {:?}", self.color_order)?;
        writeln!(f, "Initial Rot     : {:.3} rad", self.initial_rotation)?;
        writeln!(f, "Angular Offset  : {:.3} rad", self.angular_offset)?;


        Ok(())
    }
}