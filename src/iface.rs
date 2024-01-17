use crate::metric::TimeSeries;

pub struct Iface {
    pub name: String,
    pub bytes_received: TimeSeries,
    pub bytes_transmitted: TimeSeries,
}

impl Iface {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            bytes_received: TimeSeries::new(),
            bytes_transmitted: TimeSeries::new(),
        }
    }

    pub fn add(&mut self, receive: u64, transmit: u64) {
        self.bytes_received.add(receive);
        self.bytes_transmitted.add(transmit);
    }
}
