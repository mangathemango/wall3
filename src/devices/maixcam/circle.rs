use std::fmt::Display;

use glam::Vec2;

#[derive(Debug, Default, Clone, Copy)]
pub struct MaixcamCircle {
    pub position: Vec2,
    pub speed: f32,
    pub color: MaixcamCircleColor,
    pub kind: MaixcamCircleKind,
    pub area: f32
}

impl Display for MaixcamCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} circle at {:.5} with area {:.3} moving at speed {:.3}",
            self.color, self.position, self.area, self.speed
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum MaixcamCircleColor {
    #[default]
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaixcamCircleKind {
    Ring,
    #[default]
    Solid,
}
