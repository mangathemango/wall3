use std::{thread, time::Duration};

use image::GrayImage;
use rqrr::PreparedImage;
use rscam::{Camera, Config};
use crate::devices::qr::QR_READER_DOTENV_KEY;

pub struct QrDriver {
    camera: Option<Camera>,
}

impl QrDriver {
    pub fn new() -> Self {
        let mut driver = Self { camera: None };

        driver.reconnect();

        driver
    }

    pub fn reconnect(&mut self) {
        let path = match dotenv::var(QR_READER_DOTENV_KEY) {
            Ok(path) => path,
            Err(e) => {
                self.camera = None;
                return;
            }
        };
        self.camera.take();

        let mut camera = match Camera::new(&path) {
            Ok(camera) => camera,
            Err(_) => {
                self.camera = None;
                return;
            }
        };

        let config_result = camera.start(&Config {
            interval: (1, 30),
            resolution: (640, 480),
            format: b"YUYV",
            ..Default::default()
        });

        match config_result {
            Ok(_) => {
                self.camera = Some(camera);
            }
            Err(_) => {
                self.camera = None;
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        self.camera.is_some()
    }

    pub fn try_read(&mut self) -> Result<Option<String>, String> {
        let camera = match &mut self.camera {
            Some(camera) => camera,
            None => {
                thread::sleep(Duration::from_millis(100));
                return Err("Camera disconnected".into());
            }
        };

        let frame = match camera.capture() {
            Ok(frame) => frame,
            Err(e) => {
                self.camera = None;
                return Err(format!("Capture error: {}", e));
            }
        };

        // YUYV is 2 bytes per pixel
        // Convert manually into grayscale
        let mut gray = Vec::with_capacity(640 * 480);

        for chunk in frame.chunks_exact(2) {
            gray.push(chunk[0]);
        }

        let image = match GrayImage::from_raw(640, 480, gray) {
            Some(image) => image,
            None => {
                return Err("Failed to construct grayscale image".into());
            }
        };

        let mut prepared = PreparedImage::prepare(image);

        let grids = prepared.detect_grids();

        for grid in grids {
            match grid.decode() {
                Ok((_meta, content)) => {
                    return Ok(Some(content));
                }
                Err(_) => {}
            }
        }

        Ok(None)
    }
}