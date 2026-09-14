use crate::devices::{
    maixcam::circle::MaixcamCircle, 
};

#[derive(Debug, Clone)]
pub enum MaixcamMessage {
    CircleData(Vec<MaixcamCircle>),
}
