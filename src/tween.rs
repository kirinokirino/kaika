use std::fmt::Display;

pub const TWEEN_PERIOD: f32 = 1000.0 / 30.0;

#[derive(Debug, Clone)]
pub struct Tween {
    pub stopped: bool,
    pub time: f32,
    waveform: Vec<f32>,
    value: f32,
}

impl Tween {
    pub fn new(waveform: &[f32]) -> Self {
        Self {
            stopped: true,
            time: 0.0,
            waveform: waveform.to_vec(),
            value: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.value = *self
            .waveform
            .get(0)
            .expect("tween should always have a waveform");
        self.stopped = true;
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.time = offset as f32 * TWEEN_PERIOD;
        self.value();
    }

    pub fn update(&mut self, delta: f64) {
        if !self.stopped {
            self.time += 1000.0 * delta as f32;
        }
    }

    pub fn is_over(&self) -> bool {
        let time_length = (self.waveform.len()) as f32 * TWEEN_PERIOD;
        self.time >= time_length
    }

    pub fn value(&mut self) -> f32 {
        if !self.stopped {
            let step = (self.time / TWEEN_PERIOD).floor() as usize;
            if let Some(value) = self.waveform.get(step) {
                self.value = *value;
            } else {
                self.value = *self
                    .waveform
                    .last()
                    .expect("tween should always have a waveform");
                self.stopped = true;
            }
        }
        self.value
    }
}

impl Display for Tween {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Stopped: {}, Time: {}, Value: {}",
            self.stopped, self.time, self.value
        ))
    }
}
