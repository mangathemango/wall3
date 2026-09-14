pub mod command;
pub mod controller;
pub mod driver;
pub mod message;
pub mod state;

use crate::ROBOT;
use crate::devices::stm32::controller::Stm32Controller;
use crate::devices::stm32::driver::Stm32Driver;
use crate::devices::stm32::state::Stm32State;


use std::sync::mpsc;
use std::time::Duration;
const STM32_DOTENV_KEY: &str = "STM32_PATH";
const STM32_START_BYTE: u8 = 0x67;

pub fn spawn_stm32_thread() {
    std::thread::spawn(move || {
        // Create mpsc (Multi-Producer, Single-Consumer) channel for multiple different threads
        // to send commands to the stm32 in the stm32_thread
        let (sender, receiver) = mpsc::channel();

        // Sender is set globally. Other threads can clone to control the STM32
        ROBOT.set_stm32_controller(Stm32Controller::new(sender));

        let mut driver = Stm32Driver::new();
        let mut state = Stm32State::new();
        let mut last_update = std::time::Instant::now();
        loop {
            let now = std::time::Instant::now();
            let dt = now - last_update;
            if dt < Duration::from_millis(20) {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }
            state.dt = now.duration_since(last_update);
            last_update = now;

            // 🟣 1. Handle outgoing commands
            while let Ok(cmd) = receiver.try_recv() {
                state.update_command(cmd.clone());
                if let Err(e) = driver.send_command(cmd.clone()) {
                    state.log_msg = e;
                }
                state.last_command = cmd;
            }

            // 🔵 2. Handle incoming data
            match driver.try_read_frame() {
                Ok(messages) => {
                    for message in messages {
                        state.update_message(message);
                    }
                }
                Err(e) => {
                    state.log_msg = e;
                    driver.reconnect();
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
            state.driver_is_connected = driver.is_connected();
            state.publish();
        }
    });
}
