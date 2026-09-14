use std::f32::consts::PI;

use crate::math::utils::wrap_angle;
use glam::Vec2;
/// A struct representing where an object is and where it's facing
#[derive(Debug, Clone, Copy, Default)]
pub struct Pose {
    pub position: Vec2,
    pub rotation: f32, // in radians
}

impl std::fmt::Display for Pose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "((x: {:.3}, y: {:.3}), rotation: {:.2}π rad ({:.1}°))",
            self.position.x,
            self.position.y,
            self.rotation / PI,
            self.rotation.to_degrees()
        )
    }
}

impl Pose {
    pub fn difference(self, target: Pose) -> Pose {
        Pose {
            position: target.position - self.position,
            rotation: wrap_angle(target.rotation - self.rotation),
        }
    }

    pub fn scale(mut self, scale: f32) -> Pose {
        self.position *= scale;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Pose {
        self.rotation = rotation;
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Pose {
        self.position = position;
        self
    }

    pub fn to_components(&self) -> (Vec2, f32) {
        (self.position, self.rotation)
    }
}
