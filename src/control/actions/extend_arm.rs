use std::{fmt::Display, time::Duration};

use crate::{ROBOT, control::actions::Action, devices::maixcam::circle::MaixcamCircleColor};

pub type RetractArm = ExtendArm;

#[derive(Debug, Clone, Default)]
pub struct ExtendArm {
    target_position: u16,
    preset: Option<ArmExtendPreset>,
    stable_time: Duration,
    timer: Duration,
}

impl ExtendArm {
    pub fn to_position(position: u16) -> Self {
        ExtendArm {
            target_position: position,
            stable_time: Duration::from_millis(250),
            ..Default::default()
        }
    }

    pub fn to_preset(preset: ArmExtendPreset) -> Self {
        ExtendArm {
            target_position: preset.to_position(),
            preset: Some(preset),
            stable_time: Duration::from_millis(250),
            ..Default::default()
        }
    }

    pub fn to_calibration() -> Self {
        Self::to_preset(ArmExtendPreset::Calibration)
    }

    pub fn to_source() -> Self {
        Self::to_preset(ArmExtendPreset::Calibration)
    }

    pub fn to_storage_placing_spin(index: u8) -> Self {
        Self::to_preset(ArmExtendPreset::StoragePlacingSpin(index))
    }

    pub fn to_storage_placing_drop(index: u8) -> Self {
        Self::to_preset(ArmExtendPreset::StoragePlacingDrop(index))
    }

    pub fn to_storage_grabbing(index: u8) -> Self {
        Self::to_preset(ArmExtendPreset::StorageGrabbing(index))
    }

    pub fn to_placement(color: MaixcamCircleColor) -> Self {
        Self::to_preset(ArmExtendPreset::GroundPlacement(color))
    }

    pub fn to_ground_pregrab(color: MaixcamCircleColor) -> Self {
        Self::to_position(ArmExtendPreset::GroundPlacement(color).to_position() + 200)
    }

    pub fn back() -> Self {
        Self::to_preset(ArmExtendPreset::Back)
    }

    pub fn forward() -> Self {
        Self::to_preset(ArmExtendPreset::Forward)
    }
}

impl Action for ExtendArm {
    fn update(&mut self, dt: Duration) {
        let stm32_controller = ROBOT.stm32_controller();
        stm32_controller.set_horizontal_arm_position(self.target_position);
        if self
            .target_position
            .abs_diff(ROBOT.stm32_state().horizontal_arm_position)
            < 100
        {
            self.timer += dt;
        }
    }

    fn is_finished(&self) -> bool {
        self.timer > self.stable_time
    }
}

impl Display for ExtendArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Extending Arm to {:?}, ({})", self.preset, self.target_position)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArmExtendPreset {
    Back,
    Forward,
    Calibration,
    StoragePlacingSpin(u8),
    StoragePlacingDrop(u8),
    StorageGrabbing(u8),
    GroundPlacement(MaixcamCircleColor),
}

impl ArmExtendPreset {
    pub fn to_position(&self) -> u16 {
        match self { 
            Self::Forward => 0,
            Self::Back => 4800,
            Self::Calibration => 2000,

            Self::StoragePlacingSpin(index) => match *index {
                0 => 4350,
                1 => 2900,
                2 => 1700,
                _ => 0,
            },
            Self::StoragePlacingDrop(index) => match *index {
                0 => 4450,
                1 => 3650,
                2 => 1950,
                _ => 0,
            },
            Self::StorageGrabbing(index) => match *index {
                0 => 4640,
                1 => 3650,
                2 => 2000,
                _ => 0,
            },
            Self::GroundPlacement(color) => match color {
                MaixcamCircleColor::Blue => 400,
                MaixcamCircleColor::Green => 2000,
                MaixcamCircleColor::Red => 100,
            },
                                                           
        }
    }
}
