use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::dashboard::widgets::{
    compass::draw_compass, controller::draw_controller, gyro::draw_gyro, maixcam::draw_maixcam,
    odometry::draw_odometry, qr::draw_qr, stm32::draw_stm32, system::draw_system,
};

pub fn ui(f: &mut Frame) {
    let [_map_area, right_area] =
        Layout::horizontal([Constraint::Length(45), Constraint::Fill(1)]).areas(f.area());

    let [motion_area, bottom_area] =
        Layout::vertical([Constraint::Percentage(45), Constraint::Percentage(55)])
            .areas(right_area);

    let [odometry_panel, controller_panel, compass_panel] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(2),
        Constraint::Length(27),
    ])
    .areas(motion_area);

    let [system_panel, devices_area] =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
            .areas(bottom_area);

    let [top_devices, bottom_devices] =
        Layout::vertical([Constraint::Fill(1), Constraint::Fill(2)]).areas(devices_area);

    let [gyro_panel, qr_panel] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(top_devices);

    let [maixcam_panel, stm32_panel] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(bottom_devices);

    draw_system(f, system_panel);

    draw_odometry(f, odometry_panel);
    draw_controller(f, controller_panel);
    draw_compass(f, compass_panel);

    draw_gyro(f, gyro_panel);
    draw_qr(f, qr_panel);

    draw_maixcam(f, maixcam_panel);
    draw_stm32(f, stm32_panel);
}
