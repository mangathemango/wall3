pub mod circle;
pub mod driver;
pub mod message;
pub mod sample;
pub mod state;
use std::{
    time::{Duration, Instant},
};


use crate::devices::maixcam::{
    driver::MaixcamDriver, message::MaixcamMessage, state::MaixcamState,
};

const MAIXCAM_DOTENV_KEY: &str = "MAIXCAM_IP";

