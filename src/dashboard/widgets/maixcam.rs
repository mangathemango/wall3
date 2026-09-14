use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

pub fn draw_maixcam(f: &mut Frame, area: Rect) {
    let maixcam = ROBOT.maixcam_state();

    let text = format!("{}", maixcam);

    let block = Block::default()
        .title("MAIXCAM")
        .borders(Borders::ALL)
        .style(Style::default().fg(if maixcam.driver_is_connected {Color::Green} else {Color::Red}));

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);

    f.render_widget(p, area);
}