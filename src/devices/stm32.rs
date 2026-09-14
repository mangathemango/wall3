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