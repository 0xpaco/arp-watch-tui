use log::{error, info};
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::NetworkInterface;
use std::{error::Error, fs::File, io::Read, sync::mpsc::Sender};

use crate::structs::{
    arp::{ARPOperation, ArpPacket, ArpPacketBuilder},
    net::{IpAddr, MacAddr},
};

pub fn sniff(interface_name: &str, app_tx: Option<Sender<ArpPacket>>) {
    let interface_name_match = |iface: &NetworkInterface| iface.name == interface_name;

    let interfaces = pnet_datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_name_match)
        .next()
        .expect(format!("Interface not found: {}", interface_name).as_str());

    let (mut tx, mut rx) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    let mut local_mac = local_mac().unwrap();
    let mut broadcast_mac = MacAddr::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]).unwrap();
    let mut i: u8 = 0;
    loop {
        let packet = ArpPacketBuilder::default()
            .sender(
                // MacAddr::new(&[00, 00, 00, 00, 00, 00]).unwrap(),
                local_mac.clone(),
                IpAddr::new(&[192, 168, 1, 86]).unwrap(),
            )
            .target(
                broadcast_mac.clone(),
                IpAddr::new(&[192, 168, 1, i]).unwrap(),
            )
            .operation(ARPOperation::Request)
            .build();
        if let None = app_tx {
            info!(
                "Sending:\n{:?}",
                packet.raw(&mut local_mac, &mut broadcast_mac).as_slice(),
            );
        }
        tx.send_to(
            packet.raw(&mut local_mac, &mut broadcast_mac).as_slice(),
            None,
        );
        match rx.next() {
            Ok(packet) => {
                if packet.len() < 41 && packet[12..14] != [08, 06] {
                    continue;
                }

                if let Ok(packet) = ArpPacket::from(&packet[14..]) {
                    if let Some(ref app) = app_tx {
                        app.send(packet).unwrap();
                    } else {
                        info!("{:?}", packet);
                    }
                }
            }
            Err(e) => error!("Error occurred while catching packets {}", e),
        }
        i = i.checked_add(1).unwrap_or(0);
    }
}

pub fn local_mac() -> Result<MacAddr, Box<dyn Error>> {
    let mut f = File::open("/sys/class/net/enp0s3/address").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    let content = &content[..17];
    let num: Vec<u8> = content
        .split(":")
        .map(|byte| u8::from_str_radix(byte, 16).unwrap())
        .collect();
    let mac = MacAddr::new(num.as_slice())?;
    Ok(mac)
}
