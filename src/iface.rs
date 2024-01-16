#[derive(Debug)]
pub struct Receive {
    pub bytes: u64,
    pub packets: u64,
    pub errs: u64,
    pub drop: u64,
    pub fifo: u64,
    pub frame: u64,
    pub compressed: u64,
    pub multicast: u64,
}

#[derive(Debug)]
pub struct Transmit {
    pub bytes: u64,
    pub packets: u64,
    pub errs: u64,
    pub drop: u64,
    pub fifo: u64,
    pub colls: u64,
    pub carrier: u64,
    pub compressed: u64,
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub receive: Receive,
    pub transmit: Transmit,
}

impl From<&str> for Interface {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let name = parts.next().unwrap().to_string().replace(":", "");
        let receive = Receive {
            bytes: parts.next().unwrap().parse().unwrap(),
            packets: parts.next().unwrap().parse().unwrap(),
            errs: parts.next().unwrap().parse().unwrap(),
            drop: parts.next().unwrap().parse().unwrap(),
            fifo: parts.next().unwrap().parse().unwrap(),
            frame: parts.next().unwrap().parse().unwrap(),
            compressed: parts.next().unwrap().parse().unwrap(),
            multicast: parts.next().unwrap().parse().unwrap(),
        };
        let transmit = Transmit {
            bytes: parts.next().unwrap().parse().unwrap(),
            packets: parts.next().unwrap().parse().unwrap(),
            errs: parts.next().unwrap().parse().unwrap(),
            drop: parts.next().unwrap().parse().unwrap(),
            fifo: parts.next().unwrap().parse().unwrap(),
            colls: parts.next().unwrap().parse().unwrap(),
            carrier: parts.next().unwrap().parse().unwrap(),
            compressed: parts.next().unwrap().parse().unwrap(),
        };
        Self {
            name,
            receive,
            transmit,
        }
    }
}
