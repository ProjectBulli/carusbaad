use std::io::Write;
use std::io;
use crate::channel::Channel;

/*
 see https://github.com/harry1453/android-auto-headunit/blob/master/app/src/main/java/info/anodsplace/headunit/aap/protocol/messages/Messages.kt
 https://github.com/anod/headunit/blob/master/app/src/main/java/info/anodsplace/headunit/aap/AapTransport.kt
*/

pub const VERSION_REQUEST:[u8;4] = [0, 1, 0, 1];

pub fn raw_message(channel: Channel, flags: i32, type_x: i32, data: &[u8], stream: &mut impl Write) -> io::Result<usize> {
    let size = data.len();
    let total = 6 + size;
    let buffer:[u8;6] = [
        channel.asInt(),
        flags as u8,
        high_byte(size+2),
        low_byte(size + 2),
        high_byte_i32(type_x),
        low_byte_i32(type_x)];

    stream.write(&buffer)?;
    stream.write(data)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_message() {

        let mut mock = Mock::new();
        raw_message(Channel::CONTROL, 3, 1, &VERSION_REQUEST, &mut mock).unwrap();
        assert_eq!(mock.buffer, [0, 3, 0, 6, 0, 1, 0, 1, 0, 1]);
    }

    struct Mock {
        buffer: Vec<u8>
    }

    impl Mock {
        fn new() -> Mock {
            Mock { buffer: vec![] }
        }
    }

    impl Write for Mock {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            &self.buffer.extend(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
}
