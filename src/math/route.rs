use crate::math::{Pose, utils::wrap_angle};

#[derive(Debug, Clone, Default)]
pub struct Route {
    pub points: Vec<Pose>,
}

impl Route {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sample(&mut self, spacing: f32) {
        self.points = Self::sample_points(&self.points, spacing);
    }

    pub fn sample_points(points: &Vec<Pose>, spacing: f32) -> Vec<Pose> {
        assert!(
            points.len() >= 2,
            "sample_path requires at least 2 waypoints"
        );

        assert!(spacing > 0.0, "spacing must be greater than 0");

        let mut sampled_path = Vec::new();

        for pair in points.windows(2) {
            let start = pair[0];
            let end = pair[1];

            let delta = end.position - start.position;
            let distance = delta.length();

            // Prevent division weirdness
            if distance <= f32::EPSILON {
                continue;
            }

            let steps = (distance / spacing).ceil() as usize;

            for i in 0..steps {
                let t = i as f32 / steps as f32;

                // Interpolate position
                let position = start.position.lerp(end.position, t);

                // Interpolate rotation properly
                let rotation_delta = wrap_angle(end.rotation - start.rotation);

                let rotation = wrap_angle(start.rotation + rotation_delta * t);

                sampled_path.push(Pose { position, rotation });
            }
        }

        // Ensure final waypoint is included
        sampled_path.push(*points.last().unwrap());

        sampled_path
    }

    pub fn push(&mut self, point: Pose) {
        self.points.push(point);
    } 

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn first(&self) -> Option<&Pose> {
        self.points.first()
    }

    pub fn last(&self) -> Option<&Pose> {
        self.points.last()
    }
}

