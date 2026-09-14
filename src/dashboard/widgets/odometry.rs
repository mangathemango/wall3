use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::dashboard::helpers::{format_radian, paragraph};

pub fn draw_odometry(f: &mut Frame, area: Rect) {
    let odometry_state = ROBOT.odometry_state();
    let text = format!(
        "Current Twist: {}\nCurrent Pose: {}\nInitial Yaw: {}\nFPS: {:.1}",
        odometry_state.twist,
        odometry_state.current_pose,
        format_radian(odometry_state.gyro_offset),
        if odometry_state.dt.as_secs_f32() > 0.0 {
            1.0 / odometry_state.dt.as_secs_f32()
        } else {
            0.0
        } as i32
    );

    paragraph(f, area, "ODOMETRY", text);
}
