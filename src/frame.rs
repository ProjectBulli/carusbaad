use crate::channel::Channel;
use crate::flags::Flags;
use std::io;
use std::io::Write;

pub trait FrameSender {
    fn send(&mut self, channel: Channel, flags: Flags, type_x: i32, data: &[u8]) -> io::Result<usize>;
}

pub fn raw_messsage_header(channel: Channel, flags: Flags, type_x: i32, stream: &mut impl Write, size: usize) -> io::Result<usize> {
    let total = 6 + size;
    let buffer: [u8; 6] = [
        channel.as_int(),
        flags.bits(),
        high_byte(size + 2),
        low_byte(size + 2),
        high_byte_i32(type_x),
        low_byte_i32(type_x)];

    stream.write(&buffer)?;
    Ok(total)
}

/*
 fun intToBytes(value: Int, offset: Int, buf: ByteArray) {
        buf[offset] = (value / 256).toByte() // Encode length of following data:
        buf[offset + 1] = (value % 256).toByte()
    }
 */

const fn high_byte(value:usize) -> u8 {
    (value / 256) as u8
}

const fn low_byte(value:usize) -> u8 {
    (value % 256) as u8
}

const fn high_byte_i32(value:i32) -> u8 {
    (value / 256) as u8
}

const fn low_byte_i32(value:i32) -> u8 {
    (value % 256) as u8
}
