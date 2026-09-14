use crate::math::Twist;

/// A struct representing the velocities of a set of 4 mecanum wheels
#[derive(Debug, Clone, Copy, Default)]
pub struct MecanumVelocities {
    /// Front Left wheel velocity
    pub vfl: f32, 
    /// Front Right wheel velocity
    pub vfr: f32,
    /// Rear Left wheel velocity
    pub vrl: f32,
    /// Rear Right wheel velocity
    pub vrr: f32,
}

impl std::fmt::Display for MecanumVelocities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(vfl: {:.3}, vfr: {:.3}\nvrl: {:.3}, vrr: {:.3})",
            self.vfl, self.vfr, self.vrl, self.vrr
        )
    }
}

impl MecanumVelocities {
    pub fn new(vfl: f32, vfr: f32, vrl: f32, vrr: f32) -> Self {
        MecanumVelocities { vfl, vfr, vrl, vrr }
    }

    /// Clamp all mecanum velocities to [-1.0, 1.0]
    pub fn normalize(&mut self) -> Self {
        let max = self
            .vfl
            .abs()
            .max(self.vfr.abs())
            .max(self.vrl.abs())
            .max(self.vrr.abs())
            .max(1.0);
        self.vfl /= max;
        self.vfr /= max;
        self.vrl /= max;
        self.vrr /= max;
        *self
    }

    pub fn from_twist(t: Twist) -> Self {
        let vx = t.linear.x;
        let vy = t.linear.y;
        let omega = t.omega;

        let vfl = vy - vx - omega;
        let vfr = vy + vx + omega;
        let vrl = vy + vx - omega;
        let vrr = vy - vx + omega;

        MecanumVelocities { vfl, vfr, vrl, vrr }
    }

    pub fn to_twist(&self) -> Twist {
        Twist::from_mecanum_velocities(*self)
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.vfl, self.vfr, self.vrl, self.vrr]
    }
}
