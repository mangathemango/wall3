use crate::devices::stm32::STM32_START_BYTE;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Stm32Command {
    /// angle: 0-180
    SetYawServoAngle {
        angle: u8,
    },
    SetClawServoAngle {
        angle: u8,
    },
    /// text: a string of ASCII characters
    SetDisplayText {
        text: String,
    },

    /// position: values mapped from 0 (bottom/backwards) - 10000 (top/forwards)
    SetVerticalArmPosition {
        position: u16,
    },
    SetHorizontalArmPosition {
        position: u16,
    },
    #[default]
    Beep,
    /// velocities: values mapped from -10000 (-max) - 10000 (max)
    /// Note: Only set wheel velocity to target velocity, do not auto ramp velocity back to 0
    SetWheelTargetVelocities {
        velocities: [i16; 4],
    },
}

impl Stm32Command {
    fn id(&self) -> u8 {
        match self {
            Stm32Command::SetYawServoAngle { .. } => 0x01,
            Stm32Command::SetClawServoAngle { .. } => 0x02,
            Stm32Command::SetDisplayText { .. } => 0x03,
            Stm32Command::SetVerticalArmPosition { .. } => 0x06,
            Stm32Command::SetHorizontalArmPosition { .. } => 0x07,
            Stm32Command::Beep { .. } => 0x08,
            Stm32Command::SetWheelTargetVelocities { .. } => 0x09,
        }
    }

    pub fn to_packet_bytes(&self) -> Vec<u8> {
        let data = self.to_data_bytes();
        let len = data.len() as u8;
        let mut packet = vec![
            STM32_START_BYTE, // START
            self.id(),        // ID
            len,              // LEN
        ];

        packet.extend(&data); // DATA

        // checksum (XOR)
        let checksum = packet.iter().fold(0u8, |acc, x| acc ^ x);
        packet.push(checksum);

        packet
    }

    pub fn to_data_bytes(&self) -> Vec<u8> {
        match self {
            Stm32Command::SetYawServoAngle { angle } => {vec![*angle]}
            Stm32Command::SetClawServoAngle { angle } => {vec![*angle]}
            Stm32Command::SetHorizontalArmPosition { position } => {
                position.to_le_bytes().to_vec()
            }
            Stm32Command::SetVerticalArmPosition { position } => {
                position.to_le_bytes().to_vec()
            }
            Stm32Command::SetDisplayText { text } => text.as_bytes().to_vec(),

            Stm32Command::SetWheelTargetVelocities { velocities } => velocities
                .iter()
                .map(|v| v.to_le_bytes())
                .flatten()
                .collect(),
            Stm32Command::Beep => Vec::new(),
        }
    }

    pub fn to_bytes_string(&self) -> String {
        let bytes = self.to_packet_bytes();

        let hex_bytes: Vec<String> = bytes.iter().map(|b| format!("0x{:02X}", b)).collect();

        format!("[{}]", hex_bytes.join(", "))
    }
}

