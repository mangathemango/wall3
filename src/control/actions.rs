pub mod rotate_arm;
pub mod lift_arm;
pub mod general;
pub mod  stop;
pub mod express;

use std::{fmt::Display, time::Duration};

#[allow(unused_variables)]
pub trait Action: Display + Send + Sync {
    fn start(&mut self) {}

    fn update(&mut self, dt: Duration) {}

    fn is_finished(&self) -> bool {true}

    fn stop(&mut self) {}

    fn abort(&mut self) {}
}
