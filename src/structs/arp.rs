use std::{error::Error, fmt::Display};

use super::net::{IpAddr, MacAddr};

/**
Byte offset by 2
0 -> Hardware type
2 -> Protocole type
4 -> Hardware address length
6 -> Operation
8 -> Sender Hardware address (mac are 6 bytes long)
    .
    .
14 -> Sender protocol address (ip are 4 bytes long)
    .
18 -> Target Hardware address
    .
    .
24 -> Target protocol address
26  .
*/

#[derive(Debug)]
pub enum HardwareType {
    // 1 = Ether
    Ether,
    P2PP,
    HDLC,
    ADCCP,
}

#[derive(Debug)]
pub enum ProtocolType {
    // [08, 00]
    V4,
    V6,
}

#[derive(Debug)]
pub enum ARPOperation {
    Request,
    Reply,
}

#[derive(Debug)]
pub struct ArpPacket {
    pub hardware_type: HardwareType,
    pub proto_type: ProtocolType,
    pub hardware_len: usize,
    pub proto_len: usize,
    pub operation: ARPOperation,
    pub sender_mac: MacAddr,
    pub sender_ip: IpAddr,
    pub target_mac: MacAddr,
    pub target_ip: IpAddr,
}

#[derive(Debug)]
pub struct ArpParseError {
    pub cause: String,
}

impl ArpPacket {
    pub fn from(packet: &[u8]) -> Result<ArpPacket, Box<dyn Error>> {
        if packet.len() < 28 {
            return Err(Box::new(ArpParseError {
                cause: format!("Invalid packet len: {}", packet.len()),
            }));
        }
        // TODO Support other type
        if &packet[0..2] != [00, 01] {
            return Err(Box::new(ArpParseError {
                cause: format!("Invalid hardware type: {:?}", &packet[0..2]),
            }));
        }
        if &packet[2..4] != [08, 00] {
            return Err(Box::new(ArpParseError {
                cause: format!("Invalid proto version: {:?}", &packet[2..4]),
            }));
        }

        let hardware_type = HardwareType::Ether;
        let proto_type = ProtocolType::V4;
        let operation = match &packet[6..8] {
            [00, 01] => ARPOperation::Request,
            [00, 02] => ARPOperation::Reply,
            _ => panic!("Unexpected arp operation"),
        };
        let sender_mac = MacAddr::new(&packet[8..14])?;
        let sender_ip = IpAddr::new(&packet[14..18])?;
        let target_mac = MacAddr::new(&packet[18..24])?;
        let target_ip = IpAddr::new(&packet[24..28])?;
        Ok(ArpPacket {
            hardware_type,
            proto_type,
            hardware_len: 6,
            proto_len: 4,
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        })
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = vec![00, 00, 00, 00, 00, 00, 08, 00, 27, 0x7au8, 92, 65, 08, 06, 00, 01, 08, 00, 06, 04];
        let op: &[u8; 2] = match self.operation {
            ARPOperation::Request => &[00, 01],
            ARPOperation::Reply => &[00, 02]
        };
        vec.append(&mut op.to_vec());
        vec.append(&mut self.sender_mac.field.to_vec());
        vec.append(&mut self.sender_ip.field.to_vec());
        vec.append(&mut self.target_mac.field.to_vec());
        vec.append(&mut self.target_ip.field.to_vec());
        vec
    }
}

impl Display for ArpParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error occured while parsing packet [more info will come from this error asap]"
        )
    }
}
impl Error for ArpParseError {}
