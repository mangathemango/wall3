use std::{f32::consts::PI, time::Duration};

use crate::devices::{
    utils::DriverSerialPort,
    gyro::{GYRO_DOTENV_KEY, sample::GyroSample},
};

/// Driver struct to read + parse data sent from the gyro
#[derive(Debug)]
pub struct GyroDriver {
    port: DriverSerialPort,
}

impl GyroDriver {
    pub fn new() -> Self {
        GyroDriver {
            port: DriverSerialPort::from_dotenv_key(GYRO_DOTENV_KEY),
        }
    }

    pub fn reconnect(&mut self) {
        self.port = DriverSerialPort::from_dotenv_key(GYRO_DOTENV_KEY);
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_connected()
    }

    /// Reads serial bytes coming from self.port until a valid frame of bytes can be parsed into a GyroSample
    ///
    /// A valid frame of bytes coming from the HWT101CT has this structure:
    /// [0x55] [0x53] [...] [yaw] [...] [gy] [gz]
    pub fn try_read_frame(&mut self) -> Result<GyroSample, String> {
        match &mut self.port {
            DriverSerialPort::Disconnected(msg) => Err(format!("Gyro driver not active: {}", msg)),
            DriverSerialPort::Connected(port) => {
                let mut buffer = [0; 1];
                let mut frame = Vec::new();
                loop {
                    match port.read_exact(&mut buffer) {
                        Ok(_) => {}
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                            std::thread::sleep(Duration::from_millis(5));
                            continue;
                        }
                        Err(e) => {
                            self.port = DriverSerialPort::Disconnected(
                                format!("Gyro read frame error: {}", e).into(),
                            );
                            return Err(format!("Gyro read frame error: {}", e));
                        }
                    }

                    let byte = buffer[0];

                    // Always push
                    frame.push(byte);

                    // Keep frame from growing infinitely
                    if frame.len() > 22 {
                        frame.remove(0);
                    }

                    // Try to detect valid frame
                    if frame.len() >= 2 {
                        // Check header alignment
                        if frame[0] != 0x55 {
                            frame.remove(0);
                            continue;
                        }

                        if frame[1] != 0x53 {
                            frame.remove(0);
                            continue;
                        }
                    }

                    if frame.len() == 22 {
                        let yaw = i16::from_le_bytes([frame[6], frame[7]]) as f32 / 32768.0 * PI;
                        let gy =
                            i16::from_le_bytes([frame[15], frame[16]]) as f32 / 32768.0 * 2000.0;
                        let gz =
                            i16::from_le_bytes([frame[17], frame[18]]) as f32 / 32768.0 * 2000.0;

                        let sample = GyroSample { yaw, gy, gz };
                        return Ok(sample);
                    }
                }
            }
        }
    }
}
