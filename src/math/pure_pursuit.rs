use crate::math::{Pose, Route, utils::wrap_angle};

#[derive(Debug, Clone)]
pub struct PurePursuitController {
    pub current_segment: usize,
    pub lookahead_distance: f32,
}

impl PurePursuitController {
    pub fn new(lookahead_distance: f32) -> Self {
        Self {
            lookahead_distance,
            current_segment: 0,
        }
    }

    pub fn update(&mut self, current_pose: Pose, route: &Route) -> Pose {
        let current_position = current_pose.position;

        // Iterate through remaining path segments
        for i in self.current_segment..route.len() - 1 {
            let start = route.points[i];
            let end = route.points[i + 1];

            let segment = end.position - start.position;
            let segment_length = segment.length();
            if segment_length <= f32::EPSILON {
                continue;
            }

            let direction = segment / segment_length;

            // Circle center relative to segment start
            let f = start.position - current_position;

            // Solve quadratic for line-circle intersection
            let a = direction.dot(direction);
            let b = 2.0 * f.dot(direction);
            let c = f.dot(f) - self.lookahead_distance.powi(2);
            let discriminant = b * b - 4.0 * a * c;

            // No intersection
            if discriminant < 0.0 {
                continue;
            }

            let discriminant_sqrt = discriminant.sqrt();
            let t1 = (-b - discriminant_sqrt) / (2.0 * a);
            let t2 = (-b + discriminant_sqrt) / (2.0 * a);

            // Choose the farther valid intersection
            let mut chosen_t = None;
            for t in [t1, t2] {
                if t >= 0.0 && t <= segment_length {
                    chosen_t = Some(t);
                }
            }

            if let Some(t) = chosen_t {
                self.current_segment = i;
                let position = start.position + direction * t;
                // Interpolate rotation along segment
                let progress = t / segment_length;

                let rotation_delta = wrap_angle(end.rotation - start.rotation);
                let rotation = wrap_angle(start.rotation + rotation_delta * progress);

                return Pose { position, rotation };
            }
        }

        // Fallback to final pose
        *route.last().unwrap()
    }
}
