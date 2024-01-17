use std::collections::BTreeMap;

use netload::{
    iface::Iface,
    proc::{get_available_interfaces, ParseError, ProcIface},
};

fn update_nics(nics: &mut BTreeMap<String, Iface>) -> Result<(), ParseError> {
    let interfaces = ProcIface::scrape_all()?;

    for iface in interfaces {
        if let Some(nic) = nics.get_mut(&iface.name) {
            nic.add(iface.receive.bytes, iface.transmit.bytes);
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut nics: BTreeMap<String, Iface> = get_available_interfaces()
        .into_iter()
        .map(|nic| (nic.clone(), Iface::new(&nic)))
        .collect();

    update_nics(&mut nics)?;

    // Print the total bytes received on the nics
    // e.g. "enp0s31f6: 1.23 GiB"
    nics.iter().for_each(|(name, nic)| {
        println!(
            "{}: {:.2}",
            name,
            byte_unit::Byte::from(nic.bytes_received.values.last().unwrap().value)
                .get_appropriate_unit(byte_unit::UnitType::Binary)
        );
    });

    Ok(())
}
