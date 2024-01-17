#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("invalid format")]
    InvalidFormat,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn get_available_interfaces() -> Vec<String> {
    let mut interfaces = Vec::new();
    let interfaces_dir = std::path::Path::new("/sys/class/net");
    for entry in interfaces_dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        interfaces.push(name);
    }
    interfaces
}

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
pub struct ProcIface {
    pub name: String,
    pub receive: Receive,
    pub transmit: Transmit,
}

impl ProcIface {
    pub fn scrape_all() -> Result<Vec<Self>, ParseError> {
        std::fs::read_to_string("/proc/net/dev")?
            .split('\n')
            .skip(2) // skip header
            .filter(|l| !l.is_empty())
            .map(ProcIface::parse)
            .collect()
    }

    fn parse(s: &str) -> Result<Self, ParseError> {
        let mut parts = s.split_whitespace();
        let name = parts
            .next()
            .ok_or(ParseError::InvalidFormat)?
            .to_string()
            .replace(':', "");

        fn next<T: std::str::FromStr>(
            parts: &mut std::str::SplitWhitespace,
        ) -> Result<T, ParseError> {
            parts
                .next()
                .ok_or(ParseError::InvalidFormat)?
                .parse()
                .map_err(|_| ParseError::InvalidFormat)
        }

        let receive = Receive {
            bytes: next(&mut parts)?,
            packets: next(&mut parts)?,
            errs: next(&mut parts)?,
            drop: next(&mut parts)?,
            fifo: next(&mut parts)?,
            frame: next(&mut parts)?,
            compressed: next(&mut parts)?,
            multicast: next(&mut parts)?,
        };
        let transmit = Transmit {
            bytes: next(&mut parts)?,
            packets: next(&mut parts)?,
            errs: next(&mut parts)?,
            drop: next(&mut parts)?,
            fifo: next(&mut parts)?,
            colls: next(&mut parts)?,
            carrier: next(&mut parts)?,
            compressed: next(&mut parts)?,
        };

        Ok(Self {
            name,
            receive,
            transmit,
        })
    }
}
