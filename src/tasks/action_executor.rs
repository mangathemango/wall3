use std::time::{Duration, Instant};

use crate::ROBOT;
use crate::control::actions::Action;
use crate::control::routines::main::main_sequence;
use crate::control::routines::utils::{beep, set_oled_display_text_start, set_oled_display_text_stop};

pub fn spawn_action_executor_thread() {
    std::thread::spawn(|| {
        let mut last_tick = Instant::now();
        loop {
            let now = Instant::now();
            let dt = now - last_tick;
            if dt < Duration::from_millis(50) {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }

            let mut action_queue = ROBOT.action_queue_mut();
            if ROBOT.stm32_state().key1_is_pressed() {
                if action_queue.is_finished() {
                    action_queue.enqueue(set_oled_display_text_start());
                    action_queue.enqueue(main_sequence());
                } else {
                    action_queue.abort();
                    action_queue.enqueue(set_oled_display_text_stop());
                    action_queue.enqueue(beep());
                }
            }
            action_queue.update(dt);
            last_tick = now;
        }
    });
}
