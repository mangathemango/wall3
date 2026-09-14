use std::{fmt::Display, time::Duration};

use crate::{ROBOT, control::actions::Action, devices::maixcam::circle::MaixcamCircleColor};

#[derive(Clone, Copy, Debug, Default)]
pub struct RotateArm {
    pub initial_angle: Option<u8>,
    pub target_angle: u8,
    pub current_angle: u8,
    pub elapsed_time: Duration,
    pub preset: Option<ArmRotationPreset>,
    pub step_time: Duration,
}

impl RotateArm {
    pub fn to_angle(target_angle: u8) -> Self {
        Self {
            target_angle,
            step_time: Duration::from_millis(10),
            ..Default::default()
        }
    }

    pub fn slow(mut self) -> Self {
        self.step_time = Duration::from_millis(10);
        self
    }

    pub fn to_preset(target_position: ArmRotationPreset) -> Self {
        let mut result = Self::to_angle(target_position.to_angle());
        result.preset = Some(target_position);
        result
    }

    pub fn to_storage(index: u8) -> Self {
        Self::to_preset(ArmRotationPreset::Storage(index))
    }

    pub fn to_placement(color: MaixcamCircleColor) -> Self {
        Self::to_preset(ArmRotationPreset::Placement(color))
    }

    pub fn to_source() -> Self {
        Self::to_preset(ArmRotationPreset::Calibration)
    }

    pub fn to_calibration() -> Self {
        Self::to_preset(ArmRotationPreset::Calibration)
    }

    pub fn idle() -> Self {
        Self::to_preset(ArmRotationPreset::Idle)
    }
}

impl Action for RotateArm {
    fn start(&mut self) {
        self.initial_angle = ROBOT.stm32_state().yaw_servo_current_angle;
        if let Some(initial_angle) = self.initial_angle {
            self.current_angle = initial_angle;
        }
    }

    fn update(&mut self, dt: Duration) {
        if let Some(initial_angle) = self.initial_angle {
            self.current_angle = initial_angle;
            self.elapsed_time += dt;
            
            let total_steps =
                initial_angle.abs_diff(self.target_angle);
            
        let current_step =
            (self.elapsed_time.as_millis()
            / self.step_time.as_millis()) as u8;

        let clamped_step =
            current_step.min(total_steps);

        // Determine direction
        self.current_angle =
            if self.target_angle >= initial_angle {
                initial_angle + clamped_step
            } else {
                initial_angle - clamped_step
            };
            
            ROBOT.stm32_controller().set_yaw_servo(
                self.current_angle
            );
        } else {
            self.current_angle = self.target_angle;
            ROBOT.stm32_controller().set_yaw_servo(
                self.target_angle
            );
        }
    }

    fn is_finished(&self) -> bool {
        self.current_angle == self.target_angle
    }
    fn stop(&mut self) {}
}

impl Display for RotateArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rotating Arm to {:?}", self.preset)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ArmRotationPreset {
    #[default]
    Idle,
    Storage(u8),
    Placement(MaixcamCircleColor),
    Calibration,
}

impl ArmRotationPreset {
    pub fn to_angle(&self) -> u8 {
        match self {
            Self::Idle => 180,
            Self::Storage(index) => match index {
                0 => 129,
                1 => 152,
                2 => 163,
                _ => 180
            },
            Self::Placement(color) => match color {
                MaixcamCircleColor::Blue => 90,
                MaixcamCircleColor::Green => 63,
                MaixcamCircleColor::Red => 36,
            },
            Self::Calibration => 63,
        }
    }
}
