//! # Robot System Entry Point
//!
//! This binary orchestrates the full robot runtime.
//!
//! It initializes global shared state (`ROBOT`) and spawns all subsystem threads:
//! - device I/O (STM32, gyro, camera, QR)
//! - control systems (odometry, action execution)
//! - debugging interface (dashboard)
//!
//! # Execution model
//! The system is fully concurrent and runs as a set of independent loops
//! communicating through shared state and channels.

mod control;
mod dashboard;
mod devices;
mod math;
mod robot;
mod action_executor;

use std::sync::LazyLock;

use crate::control::states::odometry::spawn_odometry_thread;
use crate::dashboard::spawn_dashboard_thread;
use crate::devices::gyro::spawn_gyro_thread;
use crate::devices::maixcam::spawn_maixcam_thread;
use crate::devices::qr::spawn_qr_thread;
use crate::devices::stm32::spawn_stm32_thread;
use crate::action_executor::spawn_action_executor_thread;

use robot::Robot;

// The global ROBOT variable used to share data across different threads
static ROBOT: LazyLock<Robot> = LazyLock::new(|| Robot::new());

fn main() {
    // DEVICE THREADS
    // STM32 communication (commands + telemetry). Updates ROBOT.stm32_state and provides ROBOT.get_stm32_controller()
    spawn_stm32_thread();

    // HWTCT101 gyroscope communication. Updates ROBOT.gyro_data
    spawn_gyro_thread();

    // Maixcam communication. Updates ROBOT.maixcam_state
    spawn_maixcam_thread();

    // QR Reader communication. Updates ROBOT.qr_state
    spawn_qr_thread();

    // CONTROL THREADS
    // Odometry (position + velocity) estimation. Updates ROBOT.odometry_state
    spawn_odometry_thread();

    // Thread to queue high level actions and sequences. Updates ROBOT.action_queue
    spawn_action_executor_thread();

    // Thread to render TUI for debugging
    spawn_dashboard_thread();
}
