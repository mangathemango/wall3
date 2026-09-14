pub mod driver;
pub mod state;
use std::time::Duration;

use driver::QrDriver;

use crate::ROBOT;
const QR_READER_DOTENV_KEY: &str = "QR_READER_PATH";

