use crate::devices::{
    stm32::{STM32_DOTENV_KEY, command::Stm32Command, message::Stm32Message},
    utils::{DriverSerialPort, SerialDecoder},
};

#[derive(Debug)]
pub struct Stm32Driver {
    port: DriverSerialPort,
    decoder: SerialDecoder<Stm32Message>,
}

impl Stm32Driver {
    pub fn new() -> Self {
        Stm32Driver {
            port: DriverSerialPort::from_dotenv_key(STM32_DOTENV_KEY),
            decoder: SerialDecoder::new(),
        }
    }

    pub fn reconnect(&mut self) {
        self.port = DriverSerialPort::from_dotenv_key(STM32_DOTENV_KEY);
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_connected()
    }

    pub fn send_command(&mut self, command: Stm32Command) -> Result<usize, String> {
        let port = match &mut self.port {
            DriverSerialPort::Disconnected(msg) => {
                return Err(format!("Send command to STM32 failed: {}", msg).into());
            }
            DriverSerialPort::Connected(port) => port,
        };
        port.write(&command.to_packet_bytes())
            .map_err(|e| format!("Send command to STM32 failed: {}", e))
    }

    pub fn try_read_frame(&mut self) -> Result<Vec<Stm32Message>, String> {
        let port = match &mut self.port {
            DriverSerialPort::Disconnected(msg) => {
                return Err(format!("Read from STM32 failed: {}", msg).into());
            }
            DriverSerialPort::Connected(port) => port,
        };

        let mut buffer = [0u8; 256];

        let mut result = Vec::new();

        match port.read(&mut buffer) {
            Ok(n) => {
                for byte in &buffer[..n] {
                    if let Some(msg) = self.decoder.push_byte(*byte) {
                        result.push(msg);
                    }
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    return Ok(Vec::new());
                } else {
                    self.port =
                        DriverSerialPort::Disconnected(format!("Read from STM32 failed: {}", e));
                    return Err(format!("Read from STM32 failed: {}", e));
                }
            }
        };

        Ok(result)
    }
}
