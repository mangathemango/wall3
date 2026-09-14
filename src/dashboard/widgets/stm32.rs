use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

pub fn draw_stm32(f: &mut Frame, area: Rect) {
    let state = ROBOT.stm32_state();

    let text = format!("{}", state);

    let block = Block::default()
        .title("STM32")
        .borders(Borders::ALL)
        .style(Style::default().fg(if state.driver_is_connected {Color::Green} else {Color::Red}));

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);

    f.render_widget(p, area);
}
