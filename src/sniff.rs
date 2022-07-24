use log::{error, info, debug};
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::NetworkInterface;
use std::sync::mpsc::Sender;

use crate::structs::{
    arp::{ARPOperation, ArpPacket, HardwareType, ProtocolType},
    net::{IpAddr, MacAddr},
};

// static CUSTOM_PACKET: [u8; 42] = [
//     0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0x08u8, 0x00u8, 0x27u8, 0x7au8, 0x92u8, 0x65u8,
//     0x08u8, 0x06u8, 0x00u8, 0x01u8, 0x08u8, 0x00u8, 0x06u8, 0x04u8, 0x00u8, 0x01u8, 0x08u8, 0x00u8,
//     0x27u8, 0x7au8, 0x92u8, 0x65u8, 0x0au8, 0x00u8, 0x02u8, 0x05u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
//     0x00u8, 0x00u8, 0x0au8, 0x00u8, 0x02u8, 0x01u8,
// ];

// TODO Get router self and router mac addr then save save in a "session"
pub fn sniff(interface_name: &str, _app_tx: Sender<ArpPacket>) {
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

    // tx.send_to(&CUSTOM_PACKET, None);
    for i in 0u8..254u8 {
        let arp_packet = ArpPacket {
            hardware_type: HardwareType::Ether,
            proto_type: ProtocolType::V4,
            hardware_len: 6,
            proto_len: 4,
            operation: ARPOperation::Request,
            sender_mac: MacAddr::new(&[08, 00, 39, 122, 146, 101]).unwrap(),
            sender_ip: IpAddr::new(&[192, 168, 1, 86]).unwrap(),
            target_mac: MacAddr::new(&[00, 00, 00, 00, 00, 00]).unwrap(),
            target_ip: IpAddr::new(&[192, 168, 1, i]).unwrap(),
        };
        debug!("Sending packet to {:?}", &[192, 168, 1, i]);
        tx.send_to(arp_packet.raw().as_slice(), None);
    }

    loop {
        match rx.next() {
            Ok(packet) => {
                if packet.len() < 41 {
                    continue;
                }

                // debug!("{:?}", packet);
                // Filter none Arp packet
                if packet[12..14] != [08, 06] {
                    continue;
                }
                let packet = ArpPacket::from(&packet[14..]);
                info!("{:?}", packet);
            }
            Err(e) => error!("Error occurred while catching packets {}", e),
        }
    }
}
