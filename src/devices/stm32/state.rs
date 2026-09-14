use std::{
    fmt::Display,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::{
    ROBOT,
    devices::stm32::{command::Stm32Command, message::Stm32Message},
};

/// A struct representing the current states polled from the Stm32
#[derive(Debug, Default, Clone)]
pub struct Stm32State {
    pub driver_is_connected: bool,
    pub start_flag: Arc<AtomicBool>,

    pub yaw_servo_current_angle: Option<u8>,
    pub claw_servo_current_angle: u8,

    pub horizontal_arm_position: u16,
    pub vertical_arm_position: u16,

    pub last_command: Stm32Command,
    pub last_message: Option<Stm32Message>,
    pub log_msg: String,
    // Movement
    pub target_wheel_velocities: [i16; 4],
    pub actual_wheel_velocities: [i16; 4],
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl Stm32State {
    pub fn new() -> Stm32State {
        Stm32State::default()
    }

    pub fn update_command(&mut self, command: Stm32Command) {
        match command {
            Stm32Command::SetWheelTargetVelocities { velocities } => {
                self.target_wheel_velocities = velocities;
            }
            Stm32Command::SetClawServoAngle { angle } => {
                self.claw_servo_current_angle = angle;
            }
            Stm32Command::SetYawServoAngle { angle } => {
                self.yaw_servo_current_angle = Some(angle);
            }
            _ => (),
        }
    }

    pub fn update_message(&mut self, message: Stm32Message) {
        match message.clone() {
            Stm32Message::Log { msg } => self.log_msg = msg,
            Stm32Message::WheelVelocities { velocities } => {
                velocities.iter().enumerate().for_each(|(idx, &v)| {
                    let current = if idx % 2 == 0 { -v } else { v };
                    let target = self.target_wheel_velocities[idx];
                    let error = (current - target).abs() as f32 / target.abs() as f32;
                    if error > 1.0 {
                        // Ignore encoder spikes
                        self.actual_wheel_velocities[idx] = target;
                    } else {
                        self.actual_wheel_velocities[idx] = current;
                    }
                });
            }
            Stm32Message::Key1 => {
                self.start_flag.store(true, Ordering::Relaxed);
            }
            Stm32Message::HorizontalArmPosition { position } => {
                self.horizontal_arm_position = position
            }
            Stm32Message::VerticalArmPosition { position } => self.vertical_arm_position = position,
        };
        self.last_message = Some(message.clone());
    }

    pub fn last_command_byte_string(&self) -> String {
        self.last_command.to_bytes_string()
    }

    pub fn last_message_byte_string(&self) -> String {
        match self.last_message.clone() {
            Some(message) => message.to_bytes_string(),
            None => "[]".into(),
        }
    }

    pub fn key1_is_pressed(&self) -> bool {
        self.start_flag.swap(false, Ordering::Relaxed)
    }

    pub fn publish(&self) {
        ROBOT.set_stm32_state(self.clone());
    }
}

impl Display for Stm32State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wheel_errors: Vec<f32> = self
            .actual_wheel_velocities
            .iter()
            .zip(self.target_wheel_velocities.iter())
            .map(|(&actual, &target)| {
                if target == 0 {
                    0.0
                } else {
                    ((actual - target).abs() as f32 / target.abs() as f32) * 100.0
                }
            })
            .collect();
        write!(
            f,
            "Last command: {:?}\n({})\n\nLast message: {:?}\n({})\n\nFront Wheels: {:06} ({:5.1}%)   {:06} ({:5.1}%)\n\nBack Wheels:  {:06} ({:5.1}%)   {:06} ({:5.1}%)\nYaw servo: {}\nClaw servo: {}\nVertical Arm: {}\nHorizontal Arm: {}\nLog: {}\ndt: {:?}",
            self.last_command,
            self.last_command_byte_string(),
            self.last_message,
            self.last_message_byte_string(),
            self.actual_wheel_velocities[0],
            wheel_errors[0],
            self.actual_wheel_velocities[1],
            wheel_errors[1],
            self.actual_wheel_velocities[2],
            wheel_errors[2],
            self.actual_wheel_velocities[3],
            wheel_errors[3],
            self.yaw_servo_current_angle.unwrap_or(0),
            self.claw_servo_current_angle,
            self.vertical_arm_position,
            self.horizontal_arm_position,
            self.log_msg,
            self.dt
        )
    }
}
