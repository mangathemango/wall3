use std::{
    io,
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    ROBOT,
    control::{
        actions::{
            lift_arm::ArmLiftPreset,
            rotate_arm::ArmRotationPreset,
            stop::StopMovement,
        },
        routines::utils::beep,
    },
    dashboard::layout::ui, devices::maixcam::circle::MaixcamCircleColor,
};

pub fn start() {
    thread::spawn(|| {
        if let Err(e) = run() {
            eprintln!("TUI error: {}", e);
        }
    })
    .join()
    .unwrap();
}

fn run() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(20);
    let mut last_tick = Instant::now();

    loop {
        // exit key (optional)
        if event::poll(Duration::from_millis(10))? && let Event::Key(key) = event::read()?{
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('k') => {
                        ROBOT
                            .stm32_state()
                            .start_flag
                            .store(true, Ordering::Relaxed);
                    }
                    KeyCode::Char('b') => {
                        ROBOT.action_queue_mut().abort();
                        ROBOT.action_queue_mut().enqueue(StopMovement::new());
                        ROBOT.action_queue_mut().enqueue(beep());
                    }

                    KeyCode::Char('z') => {
                        ROBOT.stm32_controller().set_claw_servo(0);
                    }
                    KeyCode::Char('c') => {
                        ROBOT
                            .stm32_controller()
                            .set_yaw_servo(ArmRotationPreset::Idle.to_angle());
                    }
                    KeyCode::Char('v') => {
                        ROBOT
                            .stm32_controller()
                            .set_yaw_servo(ArmRotationPreset::Calibration.to_angle());
                    }
                    KeyCode::Char('f') => {
                        ROBOT
                            .stm32_controller()
                            .set_vertical_arm_position(ArmLiftPreset::Up.to_position());
                    }
                    KeyCode::Char('g') => {
                        ROBOT
                            .stm32_controller()
                            .set_vertical_arm_position(ArmLiftPreset::Ground(MaixcamCircleColor::Green).to_position());
                    }
                    _ => (),
                };
                if key.code == KeyCode::Char('q') {
                    break;
                }
                if key.code == KeyCode::Char('k') {
                    ROBOT
                        .stm32_state()
                        .start_flag
                        .store(true, Ordering::Relaxed);
                }
                if key.code == KeyCode::Char('b') {
                    ROBOT.action_queue_mut().enqueue(beep());
                }
            
        }

        if last_tick.elapsed() >= tick_rate {
            terminal.draw(ui)?;
            last_tick = Instant::now();
        }
        std::thread::sleep(Duration::from_millis(1));
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
