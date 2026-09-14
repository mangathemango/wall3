use std::sync::mpsc::Sender;

use crate::{
    devices::stm32::command::Stm32Command,
    math::{MecanumVelocities, Twist},
};

/// A controller struct used to send Pi to STM32 commands from other threads
#[derive(Debug, Clone)]
pub struct Stm32Controller {
    tx: Sender<Stm32Command>,
}

impl Stm32Controller {
    pub fn new(tx: Sender<Stm32Command>) -> Self {
        Self { tx }
    }
    pub fn send(&self, cmd: Stm32Command) {
        let _ = self.tx.send(cmd);
    }
    pub fn beep(&self) {
        self.send(Stm32Command::Beep {});
    }
    pub fn set_yaw_servo(&self, angle: u8) {
        self.send(Stm32Command::SetYawServoAngle { angle });
    }

    pub fn set_vertical_arm_position(&self, position: u16) {
        self.send(Stm32Command::SetVerticalArmPosition { position })
    }

    pub fn set_horizontal_arm_position(&self, position: u16) {
        self.send(Stm32Command::SetHorizontalArmPosition { position })
    }

    pub fn set_display_text(&self, text: String) {
        self.send(Stm32Command::SetDisplayText { text })
    }

    pub fn set_claw_servo(&self, angle: u8) {
        self.send(Stm32Command::SetClawServoAngle { angle });
    }

    pub fn set_wheel_velocities(&self, v: [i16; 4]) {
        self.send(Stm32Command::SetWheelTargetVelocities { velocities: v });
    }

    pub fn set_twist(&self, t: Twist) {
        self.set_wheel_velocities(
            MecanumVelocities::from_twist(t)
                .normalize()
                .to_array()
                .map(|v| (v * 10000.0) as i16),
        );
    }
}
