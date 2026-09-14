use sysinfo::System;

use ratatui::{
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
    layout::Rect,
    style::{Color, Style},
};

use crate::dashboard::helpers::read_temperature;

pub fn draw_system(f: &mut Frame, area: Rect) {
    let mut sys = System::new_all();
    sys.refresh_all();

    // RAM usage
    let total_mem = sys.total_memory() as f64;
    let used_mem = sys.used_memory() as f64;
    let mem_usage = (used_mem / total_mem) * 100.0;

    let text = format!(
        "SYSTEM\n\nCPU: {:.1}%\nRAM: {:.1}%\nTEMP: {}°C\n(ideal: 50-70°C)\n\nPROCS: {}",
        sys.global_cpu_usage(),
        mem_usage,
        read_temperature().unwrap_or(0.0),
        sys.processes().len()
    );

    let block = Block::default()
        .title("SYSTEM")
        .borders(Borders::ALL)
        .border_style(if sys.global_cpu_usage() > 80.0 {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        });

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);

    f.render_widget(p, area);
}