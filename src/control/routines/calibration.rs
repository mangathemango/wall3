use std::{collections::HashMap, f32::consts::FRAC_PI_2};

use crate::{ROBOT, control::{
    actions::{
        calibrate_placement::CalibratePlacement, calibrate_source::CalibrateSource, calibrate_temporary::CalibrateTemporary, general::{RuntimeSequence, Sequence}, rotate_arm::RotateArm, stop::StopMovement
    },
    landmark::Landmark,
    routines::{material_handling::pick_up_all_materials_from_ground, navigation::set_current_landmark_position},
}, devices::maixcam::circle::MaixcamCircleColor};

pub fn calibrate_at_finished_product_zone() -> Sequence {
    Sequence::new("Calibrating at source zone")
        .then(RotateArm::to_calibration())
        .then(CalibrateSource::new())
        .then(set_current_landmark_position(Landmark::FinishedProductZone))
        .then(StopMovement::new())
}

pub fn calibrate_at_temporary_storage_zone_1(color_queue: [MaixcamCircleColor; 3]) -> RuntimeSequence {
    RuntimeSequence::new(move || {
        
    
    let mut map = HashMap::new();

    map.insert(color_queue[0], MaixcamCircleColor::Blue);
    map.insert(color_queue[1], MaixcamCircleColor::Green);
    map.insert(color_queue[2], MaixcamCircleColor::Red);

    let qr_queue = ROBOT.qr_state().color_queue_1.unwrap();
    let preset_queue = qr_queue
        .into_iter()
        .map(|color| *map.get(&color).unwrap())
        .collect();


    Sequence::new("Calibrating at temporary storage zone")
        .then(RotateArm::to_calibration())
        .then(
            CalibrateTemporary::new(color_queue),
        )
        .then(pick_up_all_materials_from_ground(preset_queue))
        .then(set_current_landmark_position(
            Landmark::TemporaryStorageZone,
        ))
    })
}

pub fn calibrate_at_temporary_storage_zone_2(color_queue: [MaixcamCircleColor; 3]) -> RuntimeSequence {
    RuntimeSequence::new(move || {
    let mut map = HashMap::new();

    map.insert(color_queue[0], MaixcamCircleColor::Blue);
    map.insert(color_queue[1], MaixcamCircleColor::Green);
    map.insert(color_queue[2], MaixcamCircleColor::Red);

    let qr_queue = ROBOT.qr_state().color_queue_2.unwrap();
    let preset_queue = qr_queue
        .into_iter()
        .map(|color| *map.get(&color).unwrap())
        .collect();


    Sequence::new("Calibrating at temporary storage zone")
        .then(RotateArm::to_calibration())
        .then(
            CalibrateTemporary::new(color_queue),
        )
        .then(pick_up_all_materials_from_ground(preset_queue))
        .then(set_current_landmark_position(
            Landmark::TemporaryStorageZone,
        ))
    })
}
pub fn calibrate_at_final_processing_zone() -> Sequence {
    Sequence::new("Calibrating at final processing zone (first round)")
        .then(RotateArm::to_calibration())
        .then(
            CalibratePlacement::new()
                .while_keeping_rotation(-FRAC_PI_2)
                .with_angular_offset(1.0_f32.to_radians()),
        )
        .then(set_current_landmark_position(Landmark::FinalProcessingZone))
}

