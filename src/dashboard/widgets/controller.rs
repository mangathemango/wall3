use ratatui::{Frame, layout::Rect};

use crate::ROBOT;
use crate::dashboard::helpers::paragraph;

pub fn draw_controller(f: &mut Frame, area: Rect) {
    paragraph(
        f,
        area,
        "CONTROLLER",
        format!("{}", ROBOT.action_queue_mut()),
    );
}
