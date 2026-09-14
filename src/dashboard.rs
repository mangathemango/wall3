pub mod app;
pub mod helpers;
pub mod layout;
pub mod widgets;

pub fn spawn_dashboard_thread() {
    app::start();
}
