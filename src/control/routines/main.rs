use crate::{control::{
    actions::{
        general::{OneShot, Sequence},
        rotate_arm::RotateArm,
    },
    routines::{
        calibration::{
            calibrate_at_final_processing_zone,
            calibrate_at_finished_product_zone, calibrate_at_temporary_storage_zone_1, calibrate_at_temporary_storage_zone_2,
        },
        material_handling::{
            pick_up_all_materials_from_ground_1, pick_up_all_materials_from_ground_2, place_all_materials_on_finished_product_zone_1, place_all_materials_on_finished_product_zone_2, place_all_materials_on_ground_1, place_all_materials_on_ground_2, place_all_materials_stacked
        },
        navigation::{
            move_back_to_start, move_to_final_processing_zone, move_to_finished_product_zone,
            move_to_qr_and_find_qr, move_to_temporary_storage_zone_from_finished_product_zone,
            move_to_temporary_storage_zone_from_qr,
        },
        utils::{beep, initialize, reset_arm_state},
    },
}, devices::maixcam::circle::{MaixcamCircle, MaixcamCircleColor}};

pub fn main_sequence() -> Sequence {
    Sequence::new("Main Sequence")
        .then(beep())
        .then(initialize())
        .then(move_to_qr_and_find_qr())
        .then(move_to_temporary_storage_zone_from_qr())
        .then(calibrate_at_temporary_storage_zone_1([
            MaixcamCircleColor::Blue,
            MaixcamCircleColor::Green,
            MaixcamCircleColor::Red
        ]))
        .then(reset_arm_state())
        .then(move_to_final_processing_zone())
        .then(calibrate_at_final_processing_zone())
        .then(place_all_materials_on_ground_1())
        .then(pick_up_all_materials_from_ground_1())
        .then(reset_arm_state())
        .then(move_to_finished_product_zone())
        .then(calibrate_at_finished_product_zone())
        .then(place_all_materials_on_finished_product_zone_1())
        .then(reset_arm_state())
        .then(move_to_temporary_storage_zone_from_finished_product_zone())
        .then(calibrate_at_temporary_storage_zone_2([
            MaixcamCircleColor::Blue,
            MaixcamCircleColor::Green,
            MaixcamCircleColor::Red
        ]))
        .then(reset_arm_state())
        .then(move_to_final_processing_zone())
        .then(calibrate_at_final_processing_zone())
        .then(place_all_materials_on_ground_2())
        .then(pick_up_all_materials_from_ground_2())
        .then(reset_arm_state())
        .then(move_to_finished_product_zone())
        .then(calibrate_at_finished_product_zone())
        .then(place_all_materials_on_finished_product_zone_2())
        .then(reset_arm_state())
        .then(move_back_to_start())
}
