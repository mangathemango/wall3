use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

pub fn draw_qr(f: &mut Frame, area: Rect) {
    let qr = ROBOT.qr_state();
    let error_text = if qr.error_msg.is_empty() || qr.driver_is_connected {
        ""
    } else {
        &qr.error_msg
    };

    let text = format!("{}\n{}", qr, error_text);

    let block = Block::default()
        .title("QR")
        .borders(Borders::ALL)
        .style(Style::default().fg(if qr.driver_is_connected {
            Color::Green
        } else {
            Color::Red
        }));

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);

    f.render_widget(p, area);
}
