use std::{fmt::Display, time::Duration};

use crate::{ROBOT, control::actions::Action};

#[derive(Debug, Clone, Copy, Default)]
pub struct FindQr {
    current_speed: i16,
    current_direction: i16,
    period: Duration,
    timer: Duration,
}

impl FindQr {
    pub fn new() -> Self {
        Self {
            current_direction: 1,
            period: Duration::from_millis(1000),
            ..Default::default()
        }
    }
}

impl Action for FindQr {
    fn update(&mut self, dt: Duration) {
        self.timer += dt;
        if self.timer > self.period {
            self.timer = Duration::ZERO;
            self.current_speed += 5;
            self.current_direction *= -1;
        }


        let v = self.current_speed * self.current_direction;
        ROBOT.stm32_controller().set_wheel_velocities([v,v,v,v]);
    }

    fn is_finished(&self) -> bool {
        ROBOT.qr_state().color_queue_1.is_some() && ROBOT.qr_state().color_queue_2.is_some()
    }

    fn stop(&mut self) {
        ROBOT.stm32_controller().set_wheel_velocities([0,0,0,0]);
    }

    fn abort(&mut self) {
        ROBOT.stm32_controller().set_wheel_velocities([0,0,0,0]);
    }
}

impl Display for FindQr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Finding QR...\nMoving at velocity: {}\nfor {:?}/{:?}", 
            self.current_speed * self.current_direction, self.period, self.timer
        )
    }
}