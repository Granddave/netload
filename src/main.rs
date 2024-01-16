mod iface;

fn main() {
    let contents = std::fs::read_to_string("/proc/net/dev").unwrap();
    for line in contents.split("\n").skip(2).filter(|l| !l.is_empty()) {
        let interface = iface::Interface::from(line);
        eprintln!(
            "{}: rx: {}, tx: {}",
            interface.name, interface.receive.bytes, interface.transmit.bytes
        )
    }
}
