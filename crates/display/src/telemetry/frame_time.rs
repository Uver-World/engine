use bevy::prelude::{Res, ResMut, Resource};
use bevy::time::Time;

#[derive(Resource)]
pub struct FrameTime {
    time: Vec<f64>,
    max_samples: usize,
}

impl FrameTime {
    pub(crate) fn new(max_samples: usize) -> Self {
        FrameTime {
            time: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    fn add_sample(&mut self, frame_time: f64) {
        if self.time.len() == self.max_samples {
            self.time.remove(0);
        }
        self.time.push(frame_time);
    }

    pub(crate) fn average_fps(&self) -> f64 {
        if self.time.is_empty() {
            0.0
        } else {
            1.0 / (self.time.iter().sum::<f64>() / self.time.len() as f64)
        }
    }
}

pub fn fps_calculation_system(time: Res<Time>, mut frame_time: ResMut<FrameTime>) {
    frame_time.add_sample(time.delta_seconds_f64());
}
