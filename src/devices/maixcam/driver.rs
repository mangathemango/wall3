use crate::devices::maixcam::{
    MAIXCAM_DOTENV_KEY,
    circle::{MaixcamCircle, MaixcamCircleColor, MaixcamCircleKind},
    message::MaixcamMessage,
};

use glam::Vec2;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
struct NetworkPosition {
    x: f32,
    y: f32,
}

#[derive(Debug, Deserialize, Clone)]
struct NetworkDetection {
    #[serde(rename = "type")]
    detection_type: String,

    color: String,

    position: NetworkPosition,

    area: f32,
}

#[derive(Debug, Deserialize)]
struct NetworkPacket {
    detections: Vec<NetworkDetection>,
}

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    time::Duration,
};

#[derive(Debug)]
pub enum DriverTcpStream {
    Connected(BufReader<TcpStream>),
    Disconnected(String),
}

#[derive(Debug)]
pub struct MaixcamDriver {
    stream: DriverTcpStream,
}

impl MaixcamDriver {
    pub fn new() -> Self {
        let stream = match TcpStream::connect(dotenv::var(MAIXCAM_DOTENV_KEY).unwrap_or("".into()))
        {
            Ok(stream) => {
                stream.set_nonblocking(true).ok();

                DriverTcpStream::Connected(BufReader::new(stream))
            }

            Err(e) => DriverTcpStream::Disconnected(format!("{}", e)),
        };

        Self { stream }
    }

    pub fn reconnect(&mut self) {
        std::thread::sleep(Duration::from_secs(1));
        *self = Self::new()
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.stream, DriverTcpStream::Connected(_))
    }

    pub fn try_read_frame(&mut self) -> Result<Vec<MaixcamMessage>, String> {
        let reader = match &mut self.stream {
            DriverTcpStream::Disconnected(msg) => {
                return Err(format!("Maixcam disconnected: {}", msg));
            }

            DriverTcpStream::Connected(reader) => reader,
        };

        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => {
                self.stream = DriverTcpStream::Disconnected(format!("Nothing was read lol"));
                return Err(format!("TCP read failed"));
            }

            Ok(_) => {}

            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    return Ok(Vec::new());
                }

                self.stream = DriverTcpStream::Disconnected(format!("{}", e));

                return Err(format!("TCP read failed: {}", e));
            }
        }

        let packet: NetworkPacket = serde_json::from_str(&line).map_err(|e| e.to_string())?;

        let mut best_by_color: HashMap<String, NetworkDetection> = HashMap::new();

        for detection in packet.detections {
            best_by_color
                .entry(detection.color.clone())
                .and_modify(|existing| {
                    if detection.position.y > existing.position.y {
                        *existing = detection.clone();
                    }
                })
                .or_insert(detection);
        }

        let circles = best_by_color
            .into_values()
            .map(|d| {
                let color = match d.color.as_str() {
                    "red" => MaixcamCircleColor::Red,
                    "green" => MaixcamCircleColor::Green,
                    "blue" => MaixcamCircleColor::Blue,
                    _ => Default::default(),
                };

                let kind = match d.detection_type.as_str() {
                    "ring" => MaixcamCircleKind::Ring,
                    "solid" => MaixcamCircleKind::Solid,
                    _ => MaixcamCircleKind::default(),
                };

                MaixcamCircle {
                    position: Vec2::new(d.position.x, d.position.y),
                    speed: 0.0,
                    kind,
                    color,
                    area: d.area,
                }
            })
            .collect();

        Ok(vec![MaixcamMessage::CircleData(circles)])
    }
}
