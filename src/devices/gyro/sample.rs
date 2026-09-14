/// A data sample read from the gyroscope
#[derive(Debug, Default, Clone, Copy)]
pub struct GyroSample {
    pub yaw: f32,
    pub gy: f32,
    pub gz: f32,
}
