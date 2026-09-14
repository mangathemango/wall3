use std::f32::consts::PI;

pub fn wrap_angle(rad: f32) -> f32 {
    let mut rad = rad;
    loop {

        if rad > PI {
            rad -= PI * 2.0;
        } else if rad < -PI {
            rad += PI * 2.0;
        } else {
            break;
        }
    }
    rad
}