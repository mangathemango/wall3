use crate::devices::{stm32::STM32_START_BYTE, utils::SerialMessage};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Stm32Message {
    Log {
        msg: String,
    },
    /// Actual wheel velocities from motor encoders
    WheelVelocities {
        velocities: [i16; 4],
    },
    HorizontalArmPosition {
        position: u16,
    },
    VerticalArmPosition {
        position: u16,
    },
    #[default]
    /// running: 0x00 for false, 0x01 for true. Will set to 1 when button on STM32 is clicked
    Key1,
}

impl Stm32Message {
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0x50 => Some(Stm32Message::Log { msg: String::new() }),
            0x51 => Some(Stm32Message::WheelVelocities { velocities: [0; 4] }),
            0x52 => Some(Stm32Message::Key1),
            0x53 => Some(Stm32Message::HorizontalArmPosition { position: 0 }),
            0x54 => Some(Stm32Message::VerticalArmPosition { position: 0 }),
            _ => None,
        }
    }

    pub fn to_frame(&self) -> Vec<u8> {
        let (id, data): (u8, Vec<u8>) = match self {
            Stm32Message::Log { msg } => (0x50, msg.as_bytes().to_vec()),

            Stm32Message::WheelVelocities { velocities } => {
                let mut data = Vec::with_capacity(8);

                for velocity in velocities {
                    data.extend_from_slice(&velocity.to_le_bytes());
                }

                (0x51, data)
            }

            Stm32Message::Key1 => (0x52, vec![]),

            Stm32Message::HorizontalArmPosition { position } => {
                (0x53, position.to_le_bytes().to_vec())
            }

            Stm32Message::VerticalArmPosition { position } => {
                (0x54, position.to_le_bytes().to_vec())
            }
        };

        let mut frame = Vec::with_capacity(3 + data.len());

        frame.push(Self::START_BYTE);
        frame.push(id);
        frame.push(data.len() as u8);
        frame.extend_from_slice(&data);

        frame
    }

    pub fn to_bytes_string(&self) -> String {
        let bytes = self.to_frame();

        let formatted: Vec<String> = bytes
            .iter()
            .map(|b| format!("0x{:02X}", b))
            .collect();

        format!("[{}]", formatted.join(", "))
    }
}

impl SerialMessage for Stm32Message {
    const START_BYTE: u8 = STM32_START_BYTE;
    fn from_frame(frame: &[u8]) -> Result<Self, String> {
        if frame.len() < 4 {
            return Err("Frame too short".into());
        }

        let id = frame[1];
        let len = frame[2] as usize;

        let data = &frame[3..3 + len];

        if let Some(command) = Stm32Message::from_id(id) {
            match command {
                Stm32Message::Log { .. } => {
                    let msg = String::from_utf8_lossy(data);
                    Ok(Stm32Message::Log { msg: msg.into() })
                }

                Stm32Message::WheelVelocities { .. } => {
                    if data.len() != 8 {
                        return Err("Invalid velocity data length".into());
                    }

                    let mut velocities = [0i16; 4];
                    for i in 0..4 {
                        velocities[i] = i16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
                    }

                    Ok(Stm32Message::WheelVelocities { velocities })
                }

                Stm32Message::Key1 => {
                    if data.len() != 0 {
                        return Err("Invalid running flag length".into());
                    }

                    Ok(Stm32Message::Key1)
                }

                Stm32Message::VerticalArmPosition { .. } => {
                    if data.len() != 2 {
                        return Err("Invalid vertical arm positon length".into());
                    }
                    Ok(Self::VerticalArmPosition {
                        position: u16::from_le_bytes([data[0], data[1]]),
                    })
                }

                Stm32Message::HorizontalArmPosition { .. } => {
                    if data.len() != 2 {
                        return Err("Invalid vertical arm positon length".into());
                    }
                    Ok(Self::HorizontalArmPosition {
                        position: u16::from_le_bytes([data[0], data[1]]),
                    })
                }
            }
        } else {
            Err(format!("Unknown command ID: {}", id))
        }
    }

    fn valid_id(id: u8) -> bool {
        Self::from_id(id).is_some()
    }
}
