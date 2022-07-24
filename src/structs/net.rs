use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub struct Device {
    pub mac: MacAddr,
    pub ip: IpAddr,
}

impl Device {
    // TODO Add error
    pub fn new(mac: &[u8], ip: &[u8]) -> Result<Device, AddressParseError> {
        let mac = MacAddr::new(mac)?;
        let ip = IpAddr::new(ip)?;
        Ok(Device { mac, ip })
    }

    pub fn mac(&self) -> &MacAddr {
        &self.mac
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] @ {}", self.mac, self.ip)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MacAddr {
    pub field: Vec<u8>,
}

impl MacAddr {
    pub fn new(field: &[u8]) -> Result<MacAddr, AddressParseError> {
        if field.len() != 6 {
            return Err(AddressParseError);
        }
        Ok(MacAddr {
            field: field.to_owned(),
        })
    }
}

impl Into<Vec<u8>> for MacAddr {
    fn into(self) -> Vec<u8> {
        self.field.clone()
    }
}

impl Display for MacAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:0>2X}::{:0>2X}::{:0>2X}::{:0>2X}::{:0>2X}::{:0>2X}",
            self.field[0],
            self.field[1],
            self.field[2],
            self.field[3],
            self.field[4],
            self.field[5]
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IpAddr {
    pub field: Vec<u8>,
}

impl IpAddr {
    pub fn new(field: &[u8]) -> Result<IpAddr, AddressParseError> {
        if field.len() != 4 {
            return Err(AddressParseError);
        }
        Ok(IpAddr {
            field: field.to_owned(),
        })
    }
}

impl Into<Vec<u8>> for IpAddr {
    fn into(self) -> Vec<u8> {
        self.field.clone()
    }
}

impl Display for IpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.field[0], self.field[1], self.field[2], self.field[3],
        )
    }
}

#[derive(Debug)]
// TODO Finish this error
pub struct AddressParseError;

impl Display for AddressParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing address")
    }
}

impl Error for AddressParseError {}
