use ratatui::{Frame, layout::Rect};

use crate::ROBOT;
use crate::control::landmark::LANDMARK_SCALE;
use crate::dashboard::helpers::paragraph;
use crate::math::Pose;

pub fn draw_map(f: &mut Frame, area: Rect) {
    let odometry_state = ROBOT.odometry_state();
    let map_text = build_pose_map(odometry_state.current_pose, odometry_state.pursuit_pose, 21);
    let text = format!("{}", map_text);

    paragraph(f, area, "MAP", text);
}

fn build_pose_map(pose: Pose, target_pose: Pose, size: usize) -> String {
    let height = size.max(5) | 1;
    let width = height * 2;
    let half_w = (width / 2) as isize;
    let half_h = (height / 2) as isize;

    let robot_x = (-pose.position.x * 60.0 / LANDMARK_SCALE * 0.6).round() as isize;
    let robot_y = (pose.position.y * 30.0 / LANDMARK_SCALE * 0.6).round() as isize;

    let target_x = (-target_pose.position.x * 60.0 / LANDMARK_SCALE * 0.6).round() as isize;
    let target_y = (target_pose.position.y * 30.0 / LANDMARK_SCALE * 0.6).round() as isize;

    let mut rows = Vec::with_capacity(height);
    for row in (0..height as isize).rev() {
        let mut line = String::with_capacity(width);
        for col in 0..width as isize {
            let x = col - half_w - 17;
            let y = row - half_h + 9;
            let ch = if x == robot_x && y == robot_y {
                'X'
            } else if x == 0 && y == 0 {
                '+'
            } else if x == target_x && y == target_y {
                'O'
            } else {
                '_'
            };
            line.push(ch);
        }
        rows.push(line);
    }

    rows.join("\n")
}
