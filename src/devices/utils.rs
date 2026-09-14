use std::marker::PhantomData;

/// An enum containing a Serial port if connected, or an error message when disconnected
#[derive(Debug)]
pub enum DriverSerialPort {
    Connected(Box<dyn serialport::SerialPort>),
    Disconnected(String),
}

impl DriverSerialPort {
    pub fn from_dotenv_key(dotenv_key: &str) -> Self {
        let path = match dotenv::var(dotenv_key) {
            Ok(path) => path,
            Err(e) => {
                return DriverSerialPort::Disconnected(format!(
                    "Dotenv key {} fetch failed: {}",
                    dotenv_key, e
                ));
            }
        };
        let port = match serialport::new(&path, 115200).open() {
            Ok(port) => port,
            Err(e) => {
                return DriverSerialPort::Disconnected(format!(
                    "Open driver port {} ({}) failed: {}",
                    dotenv_key, path, e
                ));
            }
        };
        DriverSerialPort::Connected(port)
    }

    pub fn is_connected(&self) -> bool {
        match self {
            DriverSerialPort::Connected(_) => true,
            _ => false,
        }
    }
}

pub trait SerialMessage: Sized {
    const START_BYTE: u8;
    fn from_frame(frame: &[u8]) -> Result<Self, String>;
    fn valid_id(id: u8) -> bool;
}

#[derive(Debug)]
pub struct SerialDecoder<M>
where
    M: SerialMessage,
{
    frame: Vec<u8>,
    expected_len: Option<usize>,
    _marker: PhantomData<M>,
}

impl<M: SerialMessage> SerialDecoder<M> {
    pub fn new() -> Self {
        Self {
            frame: Vec::new(),
            expected_len: None,
            _marker: PhantomData,
        }
    }
    
    pub fn push_byte(&mut self, byte: u8) -> Option<M> {
        self.frame.push(byte);

        // 0: start byte check
        if self.frame.len() == 1 {
            if self.frame[0] != M::START_BYTE {
                self.frame.clear();
            }
            return None;
        }

        // 1: id validation
        if self.frame.len() == 2 {
            let id = self.frame[1];
            if !M::valid_id(id) {
                self.frame.clear();
            }
            return None;
        }

        // 2: length byte
        if self.frame.len() == 3 {
            let len = self.frame[2] as usize;
            self.expected_len = Some(3 + len + 1); // header + data + checksum
            return None;
        }

        // 3: wait until full frame
        let expected = match self.expected_len {
            Some(n) => n,
            None => {
                self.frame.clear();
                return None;
            }
        };

        if self.frame.len() < expected {
            return None;
        }

        // 4: full frame received
        let frame = std::mem::take(&mut self.frame);
        self.expected_len = None;

        // Validate checksum
        let checksum = frame[frame.len() - 1];

        let calculated_checksum = frame[..frame.len() - 1]
            .iter()
            .fold(0u8, |acc, byte| acc ^ byte);

        if checksum != calculated_checksum {
            return None;
        }

        match M::from_frame(&frame) {
            Ok(msg) => Some(msg),
            Err(_) => None,
        }
    }
}
