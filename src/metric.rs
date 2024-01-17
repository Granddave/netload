pub struct TimeSeries {
    pub values: Vec<Sample>,
}

pub struct Sample {
    pub timestamp: f64,
    pub value: u64,
}

impl Default for TimeSeries {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeSeries {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn add(&mut self, value: u64) {
        self.values.push(Sample {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            value,
        })
    }
}
