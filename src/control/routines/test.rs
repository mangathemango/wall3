use std::time::Duration;

use crate::{
    ROBOT,
    control::{
        actions::{
            extend_arm::ExtendArm,
            general::{OneShot, Sequence, WaitFor},
        },
        routines::{material_handling::{grab_material_from_ground, place_material_into_storage}, utils::{beep, reset_arm_state}},
    }, devices::maixcam::circle::MaixcamCircleColor,
};

#[allow(unused)]
pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")
        .then(reset_arm_state())
        .then(grab_material_from_ground(MaixcamCircleColor::Green))
        .then(place_material_into_storage(1))
        .then(reset_arm_state())
}

#[allow(unused)]
pub fn test_horizontal_arm() -> Sequence {
    Sequence::new("Test").then(ExtendArm::to_position(10000))
}

#[allow(unused)]
pub fn test_movement() -> Sequence {
    Sequence::new("Movement")
        .then(OneShot::new(|| {
            ROBOT
                .stm32_controller()
                .set_wheel_velocities([100, 100, 100, 100]);
        }))
        .then(WaitFor::new(Duration::from_millis(500)))
        .then(OneShot::new(|| {
            ROBOT.stm32_controller().set_wheel_velocities([0, 0, 0, 0]);
        }))
}
