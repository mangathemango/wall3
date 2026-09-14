use crate::{
    ROBOT,
    control::{
        actions::{
            general::{OneShot},
        }
    },
};

pub fn set_oled_display_text_start() -> OneShot {
    OneShot::new(|| ROBOT.stm32_controller().set_display_text("Started!".into()))
}

pub fn set_oled_display_text_stop() -> OneShot {
    OneShot::new(|| ROBOT.stm32_controller().set_display_text("Stopped".into()))
}

pub fn beep() -> OneShot {
    OneShot::new(|| ROBOT.stm32_controller().beep())
}
