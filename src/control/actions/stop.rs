use std::fmt::Display;

use crate::{ROBOT, control::actions::Action, math::Twist};

pub struct StopMovement {}

impl StopMovement {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Action for StopMovement {
    #[allow(unused)]
    fn update(&mut self, dt: std::time::Duration) {
        ROBOT.stm32_controller().set_twist(Twist::ZERO);
    }
}

impl Display for StopMovement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stopping fr\n")
    }
}
