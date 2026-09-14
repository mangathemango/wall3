use std::f32::consts::PI;
use std::time::Duration;

use glam::Vec2;

use crate::ROBOT;
use crate::control::actions::find_qr::FindQr;
use crate::control::actions::general::{OneShot, Sequence, WaitFor};
use crate::control::actions::r#move::Move;
use crate::control::landmark::Landmark;
use crate::control::routines::utils::set_oled_display_text_qr;
use crate::math::Pose;

pub fn move_to_qr_and_find_qr() -> Sequence {
    Sequence::new("Moving to Qr Zone")
        .then(Move::start_from(Landmark::Start).to(Landmark::QrZone))
        .then(FindQr::new())
        .then(set_oled_display_text_qr())
}

pub fn move_to_temporary_storage_zone_from_qr() -> Sequence {
    Sequence::new("Moving to source zone")
        .then(Move::start_from(Landmark::QrZone).to(Landmark::CentralRightCrossing))
        .then(Move::start_from(Landmark::CentralRightCrossing).to(Landmark::TemporaryStorageZone))
}



pub fn move_to_final_processing_zone() -> Sequence {
    Sequence::new("Moving to final storage zone")
        .then(Move::start_from(Landmark::TemporaryStorageZone).to(Landmark::UpperLeftTurn))
        .then(WaitFor::new(Duration::from_millis(500)))
        .then(Move::start_from(Landmark::UpperLeftTurn).to(Landmark::FinalProcessingZone))
}

pub fn move_to_finished_product_zone() -> Sequence {
    Sequence::new("Moving to source zone")
        .then(Move::start_from(Landmark::FinalProcessingZone).to(Landmark::UpperRightTurn))
        .then(WaitFor::new(Duration::from_millis(500)))
        .then(
            Move::start_from(Landmark::UpperRightTurn).to(Landmark::Custom(
                Landmark::FinishedProductZone
                    .pose()
                    .with_position(Vec2 { x: 0.10, y: 0.67 }),
            )),
        )
}
pub fn move_to_temporary_storage_zone_from_finished_product_zone() -> Sequence {
    Sequence::new("Moving to temporary storage zone")
        .then(Move::start_from(Landmark::FinishedProductZone).to(Landmark::CentralRightCrossing))
        .then(WaitFor::new(Duration::from_millis(500)))
        .then(
            Move::start_from(Landmark::CentralRightCrossing)
                .to(Landmark::TemporaryStorageZone)
                .rotate_between(0.95, 1.0),
        )
}

pub fn move_back_to_start() -> Sequence {
    Sequence::new("Moving back to start zone")
        .then(Move::start_from(Landmark::FinalProcessingZone).to(Landmark::UpperRightTurn))
        .then(WaitFor::new(Duration::from_millis(500)))
        .then(
            Move::start_from(Landmark::UpperRightTurn)
                .to(Landmark::Custom(Pose {
                    position: Vec2::new(0.05, 0.04),
                    rotation: PI,
                }))
                .to(Landmark::Custom(Pose {
                    position: Vec2::new(-0.04, 0.02),
                    rotation: PI,
                }))
                .angular_pid(-0.01, -0.0005, -0.0005, 0.02, 1.0),
        )
}

pub fn set_current_landmark(landmark: Landmark) -> OneShot {
    OneShot::new(move || {
        ROBOT.odometry_state_mut().set_current_landmark(landmark);
        ROBOT
            .odometry_state_mut()
            .set_current_rotation(landmark.pose().rotation);
    })
}

pub fn set_current_landmark_position(landmark: Landmark) -> OneShot {
    OneShot::new(move || {
        ROBOT.odometry_state_mut().set_current_landmark(landmark);
    })
}
