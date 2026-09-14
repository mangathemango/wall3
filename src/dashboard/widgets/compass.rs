use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ROBOT;

pub fn draw_compass(f: &mut Frame, area: Rect) {
    let odometry_state = ROBOT.odometry_state();

    draw_compass_inner(f, area, odometry_state.current_pose.rotation);
}

fn draw_compass_inner(f: &mut Frame, area: Rect, yaw: f32) {
    let size_x = area.width as usize - 3;
    let size_y = area.height as usize - 2;
    let mut grid = vec![vec![' '; size_x]; size_y];

    let cx = (size_x / 2) as f32;
    let cy = (size_y / 2) as f32;

    let rx = cx; // horizontal radius
    let ry = cy; // vertical radius

    // 🟣 draw circle (normalized)
    for y in 0..size_y {
        for x in 0..size_x {
            let dx = (x as f32 - cx) / rx;
            let dy = (y as f32 - cy) / ry;

            let dist = (dx * dx + dy * dy).sqrt();

            if (dist - 1.0).abs() < 0.08 {
                grid[y][x] = '•';
            }
        }
    }

    // 🟢 direction line
    let angle = yaw;

    let steps = size_x.max(size_y);

    for i in 0..steps {
        let t = i as f32 / steps as f32;

        let x = cx - t * rx * angle.sin();
        let y = cy - t * ry * angle.cos();

        let xi = x.round() as usize;
        let yi = y.round() as usize;

        if xi < size_x && yi < size_y {
            grid[yi][xi] = '│';
        }
    }

    // 🔴 center
    grid[cy as usize][cx as usize] = 'O';

    // 🧭 cardinal directions
    if cy >= 1.0 {
        grid[0][cx as usize] = 'N';
        grid[size_y - 1][cx as usize] = 'S';
    }
    if cx >= 1.0 {
        grid[cy as usize][0] = 'W';
        grid[cy as usize][size_x - 1] = 'E';
    }

    let lines: Vec<Line> = grid
        .into_iter()
        .map(|row| {
            let spans: Vec<Span> = row
                .into_iter()
                .map(|ch| match ch {
                    'N' => Span::styled("N", Style::default().fg(Color::Red)),
                    'S' => Span::styled("S", Style::default().fg(Color::Red)),
                    'E' => Span::styled("E", Style::default().fg(Color::Blue)),
                    'W' => Span::styled("W", Style::default().fg(Color::Blue)),
                    'O' => Span::styled("O", Style::default().fg(Color::Yellow)),
                    '│' => Span::styled("│", Style::default().fg(Color::Green)),
                    '•' => Span::styled("•", Style::default().fg(Color::DarkGray)),
                    _ => Span::raw(ch.to_string()),
                })
                .collect();

            Line::from(spans)
        })
        .collect();
    let p = Paragraph::new(lines).block(Block::default().title("COMPASS").borders(Borders::ALL));

    f.render_widget(p, area);
}
