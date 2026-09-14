use std::time::Duration;

/// A PID (Proportional Integral Derivative) controller is a feedback mechanism that calculates
/// the error between the desired target and the actual measured value, and adjusting the output
/// to minimize this difference.
#[derive(Debug, Default, Clone, Copy)]
pub struct PidController {
    /// How much current error contribute to correction
    kp: f32,           
    /// How much accumulated error over time (integral) contribute to correction
    ki: f32,        
    /// How much the change in error over time contribute to correction
    kd: f32,           
    // The range in which errors are deemed to be in a settled state
    tolerance: f32,    
    // Maximum accumulated integral to avoid integral overshooting
    max_integral: f32, 

    // Controller states
    last_error: f32,
    integral: f32,

    // How much time 
    pub settled_duration: Duration,
}

impl std::fmt::Display for PidController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PidController {{ kp: {:.3}, ki: {:.3}, kd: {:.3}, tolerance: {:.3}, max_integral: {:.3}, last_error: {:.3}, integral: {:.3} }}",
            self.kp,
            self.ki,
            self.kd,
            self.tolerance,
            self.max_integral,
            self.last_error,
            self.integral
        )
    }
}

impl PidController {
    pub fn new(kp: f32, ki: f32, kd: f32, tolerance: f32, max_integral: f32) -> Self {
        PidController {
            kp,
            ki,
            kd,
            max_integral,
            tolerance,

            last_error: f32::NAN,
            ..Default::default()
        }
    }

    pub fn update(&mut self, current_error: f32, dt: Duration) -> f32 {
        // Check if error is in tolerance range
        if current_error.abs() < self.tolerance {
            self.settled_duration += dt;
        } else {
            self.settled_duration = Duration::ZERO;
        }
        
        if self.last_error.is_nan() {
            self.last_error = current_error
        }

        let error_diff = (current_error - self.last_error) / dt.as_secs_f32();
        self.last_error = current_error;

        self.integral += current_error * dt.as_secs_f32();
        self.integral = self.integral.clamp(-self.max_integral, self.max_integral);

        let correction = current_error * self.kp + self.integral * self.ki + error_diff * self.kd;

        correction
    }

    pub fn is_settled_for(&self, duration: Duration) -> bool {
        self.settled_duration >= duration
    }
}
