use std::{fmt::Display, time::Duration};

use glam::Vec2;

use crate::{
    ROBOT,
    control::{actions::Action, landmark::Landmark},
    math::{PidController, Pose, PurePursuitController, Route, Twist},
};

#[derive(Debug, Clone)]
pub struct Move {
    route: Route,
    pure_pursuit: PurePursuitController,
    linear_pid: PidController,
    angular_pid: PidController,
    settle_time: Duration,
    state: MoveState,
    max_speed: f32,
    max_rotation: f32
}

impl Default for Move {
    fn default() -> Self {
        Self {
            route: Route::new(),
            pure_pursuit: PurePursuitController::new(0.005),
            linear_pid: PidController::new(1.5, 0.0, 0.3, 0.001, 0.0),
            angular_pid: PidController::new(-0.01, -0.0, -0.0003, 0.05, 1.0),
            settle_time: Duration::from_millis(300),
            state: MoveState::Translating,
            max_speed: 0.012,
            max_rotation: 0.006
        }
    }
}

impl Move {
    pub fn start_from(landmark: Landmark) -> Self {
        let mut route = Route::new();
        route.push(landmark.pose());
        Self {
            route,
            ..Default::default()
        }
    }

    pub fn to(mut self, landmark: Landmark) -> Self {
        self.route.push(landmark.pose());
        self
    }

    pub fn rotate_between(mut self, start: f32, end: f32) -> Self {
        let len = self.route.points.len();
        if len < 2 || start > end || start > 1.0 || end < 0.0 {
            return self;
        }
        let start_pose = self.route.points[len - 2];
        let end_pose = self.route.points[len - 1];

        // Remove original end pose temporarily
        self.route.points.pop();

        // Interpolate positions
        let rotate_start_position = start_pose.position.lerp(end_pose.position, start);

        let rotate_end_position = start_pose.position.lerp(end_pose.position, end);

        // Before rotation begins
        let rotate_start_pose = Pose {
            position: rotate_start_position,
            rotation: start_pose.rotation,
        };

        // Rotation completed here
        let rotate_end_pose = Pose {
            position: rotate_end_position,
            rotation: end_pose.rotation,
        };

        // Rebuild segment
        self.route.push(rotate_start_pose);
        self.route.push(rotate_end_pose);
        self.route.push(end_pose);

        self
    }

    pub fn lookahead_distance(mut self, lookahead_distance: f32) -> Self {
        self.pure_pursuit.lookahead_distance = lookahead_distance;
        self
    }

    pub fn linear_pid(
        mut self,
        kp: f32,
        ki: f32,
        kd: f32,
        tolerance: f32,
        max_integral: f32,
    ) -> Self {
        self.linear_pid = PidController::new(kp, ki, kd, tolerance, max_integral);
        self
    }

    pub fn angular_pid(
        mut self,
        kp: f32,
        ki: f32,
        kd: f32,
        tolerance: f32,
        max_integral: f32,
    ) -> Self {
        self.angular_pid = PidController::new(kp, ki, kd, tolerance, max_integral);
        self
    }

    pub fn settle_time(mut self, settle_time: Duration) -> Self {
        self.settle_time = settle_time;
        self
    }

    pub fn max_speed(mut self, max_speed: f32) -> Self {
        self.max_speed = max_speed;
        self
    }
}

impl Action for Move {
    fn start(&mut self) {
        self.route
            .sample(self.pure_pursuit.lookahead_distance / 5.0);
    }

    fn update(&mut self, dt: Duration) {
        let current_pose = ROBOT.odometry_state().current_pose;
        let initial_pose = *self.route.first().unwrap();
        let goal_pose = *self.route.last().unwrap();

        // Get next pursuit pose using Pure Pursuit algorithm
        let pursuit_pose = self.pure_pursuit.update(current_pose, &self.route);

        // Calculate linear + angular errors
        let (_, initial_angular_error) =
            current_pose.difference(initial_pose).to_components();
        let (pursuit_linear_error, pursuit_angular_error) =
            current_pose.difference(pursuit_pose).to_components();
        let (goal_linear_error, _) = current_pose.difference(goal_pose).to_components();

        // Linear direction is obtained from pursuit linear error
        let linear_output_direction = pursuit_linear_error
            .rotate(Vec2::from_angle(current_pose.rotation)) // Rotate from world -> local space
            .normalize_or_zero();

        // Linear speed is obtained from goal linear error using PID
        let linear_output_speed = self
            .linear_pid
            .update(goal_linear_error.length(), dt)
            .clamp(0.0, self.max_speed);


        if self.linear_pid.is_settled_for(self.settle_time) {
            self.state = MoveState::Rotating;
        }

        // Final linear velociy vector
        let linear_output = linear_output_direction * linear_output_speed;

        // Get angular velocity (omega) using PID
        let angular_output = match self.state {
            MoveState::Translating => self.angular_pid.update(initial_angular_error, dt),
            MoveState::Rotating => self.angular_pid.update(pursuit_angular_error, dt)
        }.clamp(-self.max_rotation, self.max_rotation);

        // Set velocity to the robot
        let target_twist = Twist::new(linear_output, angular_output);
        ROBOT.stm32_controller().set_twist(target_twist);

        // Set pursuit_pose to odometry state for debugging
        ROBOT.odometry_state_mut().pursuit_pose = pursuit_pose;
    }

    fn is_finished(&self) -> bool {
        self.linear_pid.is_settled_for(self.settle_time)
            && self.angular_pid.is_settled_for(self.settle_time)
    }

    fn stop(&mut self) {
        let stm32_controller = ROBOT.stm32_controller();
        stm32_controller.set_wheel_velocities([0, 0, 0, 0]);
    }

    fn abort(&mut self) {
        let stm32_controller = ROBOT.stm32_controller();
        stm32_controller.set_wheel_velocities([0, 0, 0, 0]);
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Moving to {:?}\n\nPure Pursuit Controller: {:?}\nLinear PID: {}\n\nAngular PID: {}\n\nSettle time: {}ms",
            self.route.last(),
            self.pure_pursuit,
            self.linear_pid,
            self.angular_pid,
            self.settle_time.as_millis()
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum MoveState {
    Translating,
    Rotating
}