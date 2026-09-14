use crate::{
    ROBOT,
    control::{
        actions::{
            extend_arm::ExtendArm,
            general::{OneShot, Sequence},
            lift_arm::LiftArm,
            rotate_arm::RotateArm,
            rotate_claw::RotateClaw,
            stop::StopMovement,
        },
        landmark::Landmark,
        routines::navigation::set_current_landmark,
    },
};

pub fn set_oled_display_text_start() -> OneShot {
    OneShot::new(|| ROBOT.stm32_controller().set_display_text("Started!".into()))
}

pub fn set_oled_display_text_stop() -> OneShot {
    OneShot::new(|| ROBOT.stm32_controller().set_display_text("Stopped".into()))
}

pub fn set_oled_display_text_qr() -> OneShot {
    OneShot::new(|| {
        let qr_text = ROBOT.qr_state().code.clone();
        if let Some(text) = qr_text {
            ROBOT.stm32_controller().set_display_text(text)
        } else {
            ROBOT.stm32_controller().set_display_text("NO QR:(".into())
        }
    })
}

pub fn beep() -> OneShot {
    OneShot::new(|| ROBOT.stm32_controller().beep())
}

pub fn initialize() -> Sequence {
    Sequence::new("Initializing...")
        .then(OneShot::new(|| ROBOT.qr_state_mut().reset()))
        .then(set_oled_display_text_start())
        .then(set_current_landmark(Landmark::Start))
        .then(StopMovement::new())
        .then(ExtendArm::forward())
        .then(LiftArm::up())
        .then(RotateArm::idle())
        .then(ExtendArm::to_calibration())
        .then(RotateClaw::open())
        .then(beep())
}

pub fn reset_arm_state() -> Sequence {
    Sequence::new("Resetting...")
        .then(StopMovement::new())
        .then(LiftArm::up())
        .then(RotateArm::idle().slow())
        .then(ExtendArm::to_calibration())
        .then(beep())
}

pub fn unleash_utter_bonfire_on_this_mortal_realm() -> Sequence {
    todo!("unspeakable")
}