use std::{fmt::Display, time::Duration};

use crate::{ROBOT, control::actions::Action, devices::maixcam::circle::MaixcamCircleColor};

pub type LowerArm = LiftArm;

#[derive(Debug, Clone, Default)]
pub struct LiftArm {
    target_position: u16,
    preset: Option<ArmLiftPreset>,
    stable_time: Duration,
    timer: Duration,
}

impl LiftArm {
    pub fn to_preset(preset: ArmLiftPreset) -> Self {
        Self {
            target_position: preset.to_position(),
            preset: Some(preset),
            stable_time: Duration::from_millis(250),
            ..Default::default()
        }
    }

    pub fn up() -> Self {
        Self::to_preset(ArmLiftPreset::Up)
    }

    pub fn to_source() -> Self {
        Self::to_preset(ArmLiftPreset::Source)
    }
    pub fn to_storage_placing() -> Self {
        Self::to_preset(ArmLiftPreset::StoragePlacing)
    }

    pub fn to_storage_grabbing() -> Self {
        Self::to_preset(ArmLiftPreset::StorageGrabbing)
    }

    pub fn to_ground(color: MaixcamCircleColor) -> Self {
        Self::to_preset(ArmLiftPreset::Ground(color))
    }

    pub fn to_stacked() -> Self {
        Self::to_preset(ArmLiftPreset::Stack)
    }
}

impl Action for LiftArm {
    fn update(&mut self, dt: Duration) {
        let stm32_controller = ROBOT.stm32_controller();
        stm32_controller.set_vertical_arm_position(self.target_position);
        if self
            .target_position
            .abs_diff(ROBOT.stm32_state().vertical_arm_position)
            < 500
        {
            self.timer += dt;
        }
    }

    fn is_finished(&self) -> bool {
        self.timer > self.stable_time
    }
}

impl Display for LiftArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Lifting Arm to {:?}\n{} -> {}",
            self.preset,
            ROBOT.stm32_state().vertical_arm_position,
            self.target_position,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArmLiftPreset {
    Up,
    StoragePlacing,
    StorageGrabbing,
    Source,
    
    Ground(MaixcamCircleColor),
    Stack,
}

impl ArmLiftPreset {
    pub fn to_position(&self) -> u16 {
        match self {
            ArmLiftPreset::Up => 0,
            ArmLiftPreset::StoragePlacing => 0,
            ArmLiftPreset::StorageGrabbing => 7500,
            ArmLiftPreset::Ground(color) => {
                match color {
                    MaixcamCircleColor::Blue => 35000,
                    MaixcamCircleColor::Green => 36400,
                    MaixcamCircleColor::Red => 36400
                }
            },
            ArmLiftPreset::Source => 10000,
            ArmLiftPreset::Stack => todo!("No need this no more"),
        }
    }
}
