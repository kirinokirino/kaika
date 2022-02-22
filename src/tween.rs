const TIMESTEP: f32 = 1000.0 / 30.0;

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

    pub fn update(&mut self, delta: f64) {
        if !self.stopped {
            self.time += delta as f32;
        }
    }

    pub fn value(&mut self) -> f32 {
        if !self.stopped {
            let step = ((1000.0 * self.time) / TIMESTEP).floor() as usize;
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
