use crate::raknet::protocol::MAGIC;
use std::convert::TryInto;
use std::net::{IpAddr, SocketAddr};
use std::num::TryFromIntError;
use std::ops::Deref;
use std::string::FromUtf8Error;

pub trait PacketBufferRead {
    fn read_magic(&self, start: usize) -> bool;
    fn read_string(&self, start: usize) -> Result<String, FromUtf8Error>;
    fn read_address(&self, start: usize) -> SocketAddr;
    fn read_u16(&self, start: usize) -> u16;
    fn read_u24(&self, start: usize) -> u32;
    fn read_u32(&self, start: usize) -> u32;
    fn read_u64(&self, start: usize) -> u64;
    fn read_u128(&self, start: usize) -> u128;
    fn read_i16(&self, start: usize) -> i16;
    fn read_i32(&self, start: usize) -> i32;
    fn read_i64(&self, start: usize) -> i64;
    fn read_i128(&self, start: usize) -> i128;
}

pub trait PacketBufferWrite {
    fn push_slice(&mut self, buff: &[u8]);
    fn push_magic(&mut self);
    fn push_string(&mut self, string: String) -> Result<(), TryFromIntError>;
    fn push_address(&mut self, addr: SocketAddr);
    fn push_u16(&mut self, num: u16);
    fn push_u24(&mut self, num: u32);
    fn push_u32(&mut self, num: u32);
    fn push_u64(&mut self, num: u64);
    fn push_u128(&mut self, num: u128);
    fn push_i8(&mut self, num: i8);
    fn push_i16(&mut self, num: i16);
    fn push_i32(&mut self, num: i32);
    fn push_i64(&mut self, num: i64);
    fn push_i128(&mut self, num: i128);
}

impl<T> PacketBufferRead for T
where
    T: Deref<Target = [u8]>,
{
    fn read_magic(&self, start: usize) -> bool {
        self[start as usize..(start + 16) as usize].to_vec() == MAGIC.to_vec()
    }

    fn read_string(&self, mut start: usize) -> Result<String, FromUtf8Error> {
        let str_buff_len = self.read_i16(start);
        start += 2;
        String::from_utf8(self[start..(start + str_buff_len as usize)].to_vec())
    }

    fn read_address(&self, start: usize) -> SocketAddr {
        //let ipv: u16 = self.read_u16(start);
        let mut parts: Vec<u8> = Vec::new();
        for part_num in 0..4 {
            let part_byte = self[start + 1 + part_num];
            parts.push(!(part_byte) & 0xff)
        }
        let port = self.read_u16(start + 5);
        SocketAddr::new(IpAddr::from([parts[0], parts[1], parts[2], parts[3]]), port)
    }

    fn read_u16(&self, start: usize) -> u16 {
        u16::from_be_bytes(self[start..start + 2].try_into().unwrap())
    }

    fn read_u24(&self, start: usize) -> u32 {
        let mut container = [0u8; 4];
        container[1..4].clone_from_slice(&self[start..start + 3]);
        let res = u32::from_be_bytes(container.try_into().unwrap());
        res
    }

    fn read_u32(&self, start: usize) -> u32 {
        u32::from_be_bytes(self[start..start + 4].try_into().unwrap())
    }

    fn read_u64(&self, start: usize) -> u64 {
        u64::from_be_bytes(self[start..start + 8].try_into().unwrap())
    }

    fn read_u128(&self, start: usize) -> u128 {
        u128::from_be_bytes(self[start..start + 16].try_into().unwrap())
    }

    fn read_i16(&self, start: usize) -> i16 {
        i16::from_be_bytes(self[start..start + 2].try_into().unwrap())
    }

    fn read_i32(&self, start: usize) -> i32 {
        i32::from_be_bytes(self[start..start + 4].try_into().unwrap())
    }

    fn read_i64(&self, start: usize) -> i64 {
        i64::from_be_bytes(self[start..start + 8].try_into().unwrap())
    }

    fn read_i128(&self, start: usize) -> i128 {
        i128::from_be_bytes(self[start..start + 16].try_into().unwrap())
    }
}

impl PacketBufferWrite for Vec<u8> {
    fn push_slice(&mut self, buff: &[u8]) {
        self.extend_from_slice(buff);
    }

    fn push_magic(&mut self) {
        self.push_slice(&MAGIC);
    }

    fn push_string(&mut self, string: String) -> Result<(), TryFromIntError> {
        self.push_u16(string.len().try_into()?);
        self.push_slice(string.as_bytes());
        Ok(())
    }

    fn push_address(&mut self, addr: SocketAddr) {
        if addr.is_ipv4() {
            self.push(0x04);
        } else {
            self.push(0x06);
        }
        let ip = addr.ip().to_string();
        let ip_parts: Vec<&str> = ip.split(".").collect();
        for ip_part in ip_parts {
            let ip_part_byte = u8::from_str_radix(ip_part, 10).unwrap();
            self.push(!(ip_part_byte) & 0xff);
        }
        self.push_u16(addr.port());
    }

    fn push_u16(&mut self, num: u16) {
        self.push_slice(&num.to_be_bytes());
    }

    fn push_u24(&mut self, num: u32) {
        let bytes = &num.to_be_bytes()[1..4];
        self.push_slice(bytes);
    }

    fn push_u32(&mut self, num: u32) {
        self.push_slice(&num.to_be_bytes());
    }

    fn push_u64(&mut self, num: u64) {
        self.push_slice(&num.to_be_bytes());
    }

    fn push_u128(&mut self, num: u128) {
        self.push_slice(&num.to_be_bytes());
    }

    fn push_i8(&mut self, num: i8) {
        self.push_slice(&num.to_be_bytes());
    }

    fn push_i16(&mut self, num: i16) {
        self.push_slice(&num.to_be_bytes());
    }

    fn push_i32(&mut self, num: i32) {
        self.push_slice(&num.to_be_bytes());
    }

    fn push_i64(&mut self, num: i64) {
        self.push_slice(&num.to_be_bytes());
    }

    fn push_i128(&mut self, num: i128) {
        self.push_slice(&num.to_be_bytes());
    }
}
