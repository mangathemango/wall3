use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::ROBOT;
use crate::devices::gyro::state::GyroState;

pub fn draw_gyro(f: &mut Frame, area: Rect) {
    let g = ROBOT.gyro_state();
    draw_gyro_text(f, area, &*g);
}

fn draw_gyro_text(f: &mut Frame, area: Rect, g: &GyroState) {
    let color = if !g.driver_is_connected {
        Color::Red
    } else {
        Color::Green
    };

    let text = format!("{}", g);

    let block = Block::default()
        .title("GYRO")
        .borders(Borders::ALL)
        .style(Style::default().fg(color));

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);

    f.render_widget(p, area);
}
