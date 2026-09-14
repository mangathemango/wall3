use std::{
    cmp::Ordering,
    f32::consts::{FRAC_PI_2, PI},
    fmt::Display,
    time::Duration,
};

use glam::Vec2;

use crate::{
    ROBOT,
    control::{actions::Action, landmark::Landmark},
    math::{PidController, Pose, Twist},
};

// This command assumes the arm is on the right side by hard code bc we only do that lmoa

pub struct CalibrateSource {
    // Configs
    move_time: Duration,
    circle_stable_time: Duration,
    circle_stable_speed: f32,

    // States
    state: CalibrateState,
    timer: Duration,
    initial_rotation: f32,
    linear_pid: PidController,
    angular_pid: PidController,
}

impl CalibrateSource {
    pub fn new() -> Self {
        Self {
            move_time: Duration::from_millis(3500),
            circle_stable_time: Duration::from_millis(500),
            circle_stable_speed: 0.1,

            state: CalibrateState::WaitingForUnstable,
            timer: Duration::ZERO,
            linear_pid: PidController::new(0.006, 0.0001, 0.0, 0.05, 1.0),
            angular_pid: PidController::new(-0.004, -0.0005, -0.0005, 0.02, 1.0),

            initial_rotation: 0.0,
        }
    }
}

impl Action for CalibrateSource {
    fn start(&mut self) {
        self.initial_rotation = PI;
    }

    fn update(&mut self, dt: Duration) {
        let circles = ROBOT.maixcam_state().circles.clone();
        let chosen_circle = circles.iter().max_by(|a, b| {
            a.position
                .y
                .partial_cmp(&b.position.y)
                .unwrap_or(Ordering::Equal)
        });
        match self.state {
            CalibrateState::WaitingForUnstable => {
                ROBOT.stm32_controller().set_twist(Twist::ZERO);
                if let Some(circle) = chosen_circle {
                    if circle.speed > self.circle_stable_speed {
                        self.timer += dt;
                        if self.timer > self.circle_stable_time {
                            self.state = CalibrateState::WaitingForStable;
                        }
                    } else {
                        self.timer = Duration::ZERO;
                    }
                }
            }

            CalibrateState::WaitingForStable => {
                ROBOT.stm32_controller().set_twist(Twist::ZERO);
                if let Some(circle) = chosen_circle {
                    if circle.speed < self.circle_stable_speed {
                        self.timer += dt;
                        if self.timer > self.circle_stable_time {
                            self.state = CalibrateState::MovingToTarget;
                        }
                    } else {
                        self.timer = Duration::ZERO;
                    }
                }
            }
            CalibrateState::MovingToTarget => {
                self.timer += dt;

                if self.timer > self.move_time {
                    ROBOT.stm32_controller().set_twist(Twist::ZERO);
                    self.state = CalibrateState::WaitingForStable;
                    self.timer = Duration::ZERO;
                    return;
                }
                if let Some(circle) = chosen_circle {
                    let current_rotation = ROBOT.odometry_state().current_pose.rotation;
                    // Move the robot linearly so that the circle ends up in the target position while keeping the initial rotation stable
                    let current_state = Pose {
                        position: circle.position,
                        rotation: current_rotation,
                    };

                    let target_state = Pose {
                        position: Vec2::new(0.47, 0.62),
                        rotation: self.initial_rotation,
                    };

                    let (linear_error, angular_error) =
                        current_state.difference(target_state).to_components();

                    // Get PID outputs from motion_policy
                    let linear_output_direction = linear_error.normalize_or_zero();
                    let linear_output_speed = self
                        .linear_pid
                        .update(linear_error.length(), dt)
                        .clamp(0.0, 0.01);
                    let mut linear_output = linear_output_speed * linear_output_direction;

                    linear_output = linear_output.rotate(Vec2::from_angle(-FRAC_PI_2));

                    let angular_output = self.angular_pid.update(angular_error, dt);
                    let target_twist = Twist::new(linear_output, angular_output);
                    ROBOT.stm32_controller().set_twist(target_twist);
                } else {
                    ROBOT.stm32_controller().set_twist(Twist::ZERO);
                }
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.state == CalibrateState::MovingToTarget
            && self.linear_pid.is_settled_for(Duration::from_millis(1000))
            && self.angular_pid.is_settled_for(Duration::from_millis(100))
    }

    fn stop(&mut self) {
        ROBOT.stm32_controller().set_twist(Twist::ZERO);
        ROBOT
            .odometry_state()
            .set_current_pose(Landmark::FinishedProductZone.pose());
    }
}

impl Display for CalibrateSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:?}: {:?}/{:?}\n\nLinear PID: {}\n\nAngular PID: {}",
            self.state,
            self.timer,
            match self.state {
                CalibrateState::WaitingForStable | CalibrateState::WaitingForUnstable =>
                    self.circle_stable_time,
                CalibrateState::MovingToTarget => self.move_time,
            },
            self.linear_pid,
            self.angular_pid,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CalibrateState {
    WaitingForUnstable,
    WaitingForStable,
    MovingToTarget,
}
