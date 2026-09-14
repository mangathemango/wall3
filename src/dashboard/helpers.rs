use std::f32::consts::PI;

use ratatui::{
    Frame, layout::Rect, widgets::{Block, Borders, Paragraph, Wrap}
};

pub fn paragraph(f: &mut Frame, area: Rect, title: &str, text: String) {
    let block = Block::default().title(title).borders(Borders::ALL);
    let p = Paragraph::new(text).wrap(Wrap {trim: true}).block(block);
    f.render_widget(p, area);
}

pub fn format_radian(rad: f32) -> String {
    format!("{:.4}π rad ({:.4}°)", rad / PI, rad.to_degrees())
}

pub fn read_temperature() -> Option<f32> {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .ok()?
            .trim()
            .parse::<f32>()
            .ok()
            .map(|v| v / 1000.0)
    }

    #[cfg(target_os = "windows")]
    {
        None // Windows is cursed for temps without extra APIs
    }

    #[cfg(target_os = "macos")]
    {
        None // same story unless using IOKit bindings
    }
}