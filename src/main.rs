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
mod tasks;

use std::{str::FromStr, sync::LazyLock, thread, time::Duration};

use robot::Robot;
use tokio::runtime::Runtime;

use crate::{control::actions::express::{Express, Expression}, devices::llm::driver::LLMDriver, tasks::{action_executor::spawn_action_executor_thread, dashboard::spawn_dashboard_thread, gyro::spawn_gyro_thread, maixcam::spawn_maixcam_thread, odometry::spawn_odometry_thread, qr::spawn_qr_thread, stm32::spawn_stm32_thread}};

// The global ROBOT variable used to share data across different threads
static ROBOT: LazyLock<Robot> = LazyLock::new(Robot::new);

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
    // spawn_dashboard_thread();

    thread::spawn(|| {
        let runtime = Runtime::new().unwrap();

        runtime.block_on(async {
            let llm_driver = LLMDriver::new();
            loop {
                let message = "Wander in your own thoughts";
                tokio::time::sleep(Duration::from_secs(10)).await;
                let response = llm_driver
                    .send_message(message)
                    .await
                    .unwrap_or("Something went wrong".into());

                println!("Wall3 thought: {response}");

                let expression_str = response
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap_or(&"Dead")
                    .to_string();
                let expression = Expression::from_str(expression_str.as_str()).unwrap_or(Expression::Dead);
                ROBOT.action_queue_mut().enqueue(
                    Express::new(expression)
                );

            }
        });
    });
    loop {
        std::thread::sleep(Duration::from_mins(10));
    }
}
