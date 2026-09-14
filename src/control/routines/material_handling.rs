use std::time::Duration;

use crate::{
    ROBOT,
    control::actions::{
        extend_arm::{ExtendArm, RetractArm},
        general::{RuntimeSequence, Sequence, WaitFor, WaitUntil},
        lift_arm::{LiftArm, LowerArm},
        rotate_arm::RotateArm,
        rotate_claw::RotateClaw,
    },
    devices::maixcam::circle::MaixcamCircleColor,
};

pub fn place_all_materials_on_finished_product_zone_1() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.qr_state().color_queue_1;

        let mut sequence = Sequence::new("Picking up materials from source (First sequence)");
        if let Some(queue) = queue {
            queue.into_iter().enumerate().for_each(|(index, color)| {
                sequence.enqueue(wait_and_place_material_on_finished_product_zone(index as u8, color));
            });
        }
        sequence
    })
}

pub fn place_all_materials_on_finished_product_zone_2() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.qr_state().color_queue_2;

        let mut sequence = Sequence::new("Picking up materials from source (First sequence)");
        if let Some(queue) = queue {
            queue.into_iter().enumerate().for_each(|(index, color)| {
                sequence.enqueue(wait_and_place_material_on_finished_product_zone(index as u8, color));
            });
        }
        sequence
    })
}



pub fn wait_and_place_material_on_finished_product_zone(storage: u8, color: MaixcamCircleColor) -> Sequence {
    Sequence::new(format!("Waiting to pick up {:?} material from source", color).as_str())
        .then(LiftArm::up())
        .then(ExtendArm::to_source())
        .then(RotateArm::to_source())
        .then(RotateClaw::open())
        .then(WaitUntil::new(
            format!("{:?} material is in frame", color).as_str(),
            Duration::from_millis(500),
            move || {
                let circle_color = color.clone();
                let maixcam = ROBOT.maixcam_state();
                let circle = maixcam.find_ring(&circle_color);
                if let Some(circle) = circle {
                    return circle.speed < 0.1; 
                }
                false
            },
        ))
        .then(grab_material_from_storage(storage))
        .then(place_material_finished_product_zone())
}

pub fn place_material_into_storage(index: u8) -> Sequence {
    Sequence::new(format!("Placing material into storage {}", index).as_str())
        .then(LiftArm::up())
        .then(ExtendArm::to_storage_placing_spin(index))
        .then(RotateArm::to_storage(index).slow())
        .then(ExtendArm::to_storage_placing_drop(index))
        .then(LiftArm::to_storage_placing())
        .then(WaitFor::new(Duration::from_millis(250)))
        .then(RotateClaw::soft_open())
        .then(LiftArm::up())
        .then(RotateClaw::open())
}


pub fn place_all_materials_on_ground_1() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.qr_state().color_queue_1;

        let mut sequence = Sequence::new("Placing materials on the ground (First sequence)");
        if let Some(queue) = queue {
            queue.into_iter().enumerate().for_each(|(index, color)| {
                sequence.enqueue(grab_material_from_storage(index as u8));
                sequence.enqueue(place_material_ground(color));
            });
        }
        sequence
    })
}

pub fn grab_material_from_storage(index: u8) -> Sequence {
    Sequence::new(format!("Grabbing material from storage {}", index).as_str())
        .then(LiftArm::up())
        .then(RetractArm::back())
        .then(RotateClaw::soft_open())
        .then(RotateArm::to_storage(index).slow())
        .then(LowerArm::to_storage_grabbing())
        .then(ExtendArm::to_storage_grabbing(index))
        .then(RotateClaw::close())
        .then(LiftArm::up())
}

pub fn place_material_ground(color: MaixcamCircleColor) -> Sequence {
    Sequence::new(format!("Placing {:?} material on the ground", color).as_str())
        .then(RotateArm::to_placement(color).slow())
        .then(ExtendArm::to_placement(color))
        .then(LowerArm::to_ground(color))
        .then(RotateClaw::open())
}

pub fn pick_up_all_materials_from_ground_1() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.qr_state().color_queue_1.unwrap_or_default();
        pick_up_all_materials_from_ground(queue)
    })
}

pub fn pick_up_all_materials_from_ground(queue: Vec<MaixcamCircleColor>) -> Sequence {
    let mut sequence = Sequence::new("Placing materials on the ground (First sequence)");
    queue.into_iter().enumerate().for_each(|(index, color)| {
        sequence.enqueue(grab_material_from_ground(color));
        sequence.enqueue(place_material_into_storage(index as u8));
    });
    sequence
}

pub fn grab_material_from_ground(color: MaixcamCircleColor) -> Sequence {
    Sequence::new(format!("Grabbing {:?} material from ground", color).as_str())
        .then(LiftArm::up())
        .then(RotateArm::to_placement(color).slow())
        .then(ExtendArm::to_ground_pregrab(color))
        .then(LowerArm::to_ground(color))
        .then(ExtendArm::to_placement(color))
        .then(RotateClaw::close())
}



pub fn place_all_materials_on_ground_2() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.qr_state().color_queue_2;

        let mut sequence = Sequence::new("Placing all materials on the ground (Second sequence)");
        if let Some(queue) = queue {
            queue.into_iter().enumerate().for_each(|(index, color)| {
                sequence.enqueue(grab_material_from_storage(index as u8));
                sequence.enqueue(place_material_ground(color));
            });
        }
        sequence
    })
}


pub fn pick_up_all_materials_from_ground_2() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.qr_state().color_queue_2;

        let mut sequence = Sequence::new("Placing materials on the ground (Second sequence)");
        if let Some(queue) = queue {
            queue.into_iter().enumerate().for_each(|(index, color)| {
                sequence.enqueue(grab_material_from_ground(color));
                sequence.enqueue(place_material_into_storage(index as u8));
            });
        }
        sequence
    })
}

pub fn place_all_materials_stacked() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.qr_state().color_queue_2;

        let mut sequence = Sequence::new("Stacking all materials");
        if let Some(queue) = queue {
            queue.into_iter().enumerate().for_each(|(index, color)| {
                sequence.enqueue(grab_material_from_storage(index as u8));
                sequence.enqueue(place_material_stacked(color));
            });
        }
        sequence
    })
}

fn place_material_finished_product_zone() -> Sequence {
    Sequence::new(format!("Place material on finished product zone").as_str())
        .then(LiftArm::up())
        .then(ExtendArm::to_source())
        .then(RotateArm::to_source())
        .then(LowerArm::to_source())
        .then(RotateClaw::open())
}

fn place_material_stacked(color: MaixcamCircleColor) -> Sequence {
    Sequence::new(format!("Stacking {:?} material", color).as_str())
        .then(RotateArm::to_placement(color).slow())
        .then(ExtendArm::to_placement(color))
        .then(LowerArm::to_stacked())
        .then(RotateClaw::open())
}
