pub mod r#move;
pub mod rotate_arm;
pub mod rotate_claw;
pub mod lift_arm;
pub mod extend_arm;
pub mod calibrate_placement;
pub mod calibrate_temporary;
pub mod calibrate_source;
pub mod general;
pub mod find_qr;
pub mod  stop;

use std::{fmt::Display, time::Duration};

#[allow(unused_variables)]
pub trait Action: Display + Send + Sync {
    fn start(&mut self) {}

    fn update(&mut self, dt: Duration) {}

    fn is_finished(&self) -> bool {true}

    fn stop(&mut self) {}

    fn abort(&mut self) {}
}
