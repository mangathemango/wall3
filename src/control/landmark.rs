pub const LANDMARK_SCALE: f32 = 0.05;

use std::f32::consts::{FRAC_PI_2, PI};

use glam::Vec2;

use crate::math::Pose;

#[derive(Debug, Clone, Copy)]
pub enum Landmark {
    Start,
    QrZone,
    FinishedProductZone,
    CentralRightCrossing,
    TemporaryStorageZone,
    UpperLeftTurn,
    FinalProcessingZone,
    UpperRightTurn,

    Custom(Pose),
}

impl Landmark {
    pub fn pose(&self) -> Pose {
        let normalized = match self {
            Landmark::Start => Pose {
                position: Vec2::new(0.0, 0.0),
                rotation: PI,
            },

            Landmark::QrZone => Pose {
                position: Vec2::new(0.10, 0.38),
                rotation: PI,
            },

            Landmark::FinishedProductZone => Pose {
                position: Vec2::new(0.10, 0.75),
                rotation: PI,
            },

            Landmark::CentralRightCrossing => Pose {
                position: Vec2::new(0.1, 0.50),
                rotation: FRAC_PI_2,
            },

            Landmark::TemporaryStorageZone => Pose {
                position: Vec2::new(0.94, 0.50),
                rotation: 0.0,
            },

            Landmark::UpperLeftTurn => Pose {
                position: Vec2::new(0.94, 0.90),
                rotation: -FRAC_PI_2,
            },

            Landmark::FinalProcessingZone => Pose {
                position: Vec2::new(0.55, 0.90),
                rotation: -FRAC_PI_2,
            },

            Landmark::UpperRightTurn => Pose {
                position: Vec2::new(0.10, 0.90),
                rotation: PI,
            },
            Landmark::Custom(pose) => *pose,
        };
        normalized.scale(LANDMARK_SCALE)
    }
}
