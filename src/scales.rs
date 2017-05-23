#[derive(Copy, Clone)]
pub struct Scales {
    pub h: (f64, f64),
    pub v: (f64, f64),
    pub n_samples: u32,
}

impl Scales {
    pub fn get_width(&self) -> f64 {
        self.h.1 - self.h.0
    }

    pub fn get_height(&self) -> f64 {
        self.v.1 - self.v.0
    }

    pub fn from_sampling_rate(&mut self, rate: ::redpitaya_scpi::acquire::SamplingRate) {
        let duration = rate.get_buffer_duration();
        let h1 = (duration.as_secs() * 1_000_000 + duration.subsec_nanos() as u64 / 1_000) as f64;

        self.h.1 = h1;
    }

    pub fn v_div(&self) -> f64 {
        (self.v.1 - self.v.0) / 10.0
    }

    pub fn h_div(&self) -> f64 {
        (self.h.1 - self.h.0) / 10.0
    }

    pub fn sample_to_ms(&self, sample: u32) -> f64 {
        sample as f64 / self.n_samples as f64 * self.h.1
    }
}
