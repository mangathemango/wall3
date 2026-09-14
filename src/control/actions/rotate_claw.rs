use std::{fmt::Display, time::Duration};

use crate::{
    ROBOT,
    control::actions::Action,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct RotateClaw {
    pub initial_angle: u8,
    pub target_angle: u8,
    pub elapsed_time: Duration,
    pub preset: Option<ClawRotationPreset>
}

impl RotateClaw {
    pub fn to(target_position: ClawRotationPreset) -> Self {
        Self {
            target_angle: target_position.to_angle(),
            ..Default::default()
        }
    }

    pub fn open() -> Self {Self::to(ClawRotationPreset::Open)}
    pub fn close() -> Self {Self::to(ClawRotationPreset::Close)}
    pub fn soft_open() -> Self {Self::to(ClawRotationPreset::SoftOpen)}
}

#[allow(unused_variables)]
impl Action for RotateClaw {
    fn start(&mut self) {
        self.initial_angle = ROBOT.stm32_state().claw_servo_current_angle;
        let stm32_controller = ROBOT.stm32_controller();
        stm32_controller.set_claw_servo(self.target_angle);
    }

    fn update(&mut self, dt: Duration) {
        let stm32_controller = ROBOT.stm32_controller();
        stm32_controller.set_claw_servo(self.target_angle);
        self.elapsed_time += dt;
    }

    fn is_finished(&self) -> bool {
        self.elapsed_time > Duration::from_millis(400)
    }
}

impl Display for RotateClaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rotate Claw to {:?}", self.preset)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ClawRotationPreset {
    #[default]
    Open,
    SoftOpen,
    Close,
}

impl ClawRotationPreset {
    pub fn to_angle(&self) -> u8 {
        match self {
            ClawRotationPreset::Open => 100,
            ClawRotationPreset::SoftOpen => 20,
            ClawRotationPreset::Close => 0,
        }
    }
}
