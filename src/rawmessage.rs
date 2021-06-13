use std::io::{Write, Read};
use std::io;
use crate::channel::Channel;
use rustls::{ClientConfig, ClientSession, Session};
use std::sync::Arc;
use webpki::DNSNameRef;
use bytebuffer::ByteBuffer;
use std::convert::TryInto;
use crate::rtls::{DummyServerCertVerifier, load_key_and_cert};

/*
 see https://github.com/harry1453/android-auto-headunit/blob/master/app/src/main/java/info/anodsplace/headunit/aap/protocol/messages/Messages.kt
 https://github.com/anod/headunit/blob/master/app/src/main/java/info/anodsplace/headunit/aap/AapTransport.kt
*/

pub fn version_request( stream: &mut impl Write) -> io::Result<usize> {
    return raw_message(Channel::CONTROL, 3, 1, &[0, 1, 0, 1], stream);
}

pub fn status_ok( stream: &mut impl Write) -> io::Result<usize> {
    return raw_message(Channel::CONTROL, 3, 4, &[8, 0], stream);
}

pub fn tls_handshake(tls_session: &mut ClientSession, stream: &mut impl Write)  -> io::Result<usize> {
    let buffer = &mut ByteBuffer::new();
    let mut tls_stream =  rustls::Stream::new(tls_session,  buffer);
    tls_stream.flush();
    let foo = buffer.to_bytes();
    let handshake_data = foo.as_slice();

    return raw_message(Channel::CONTROL, 3, 3, handshake_data, stream)
}

pub fn create_tlsclient<'a>() ->  ClientSession {
    let mut client_config = ClientConfig::new();

    client_config.enable_sni = false;
    client_config.enable_early_data = true;
    client_config.dangerous().set_certificate_verifier(Arc::new( DummyServerCertVerifier {}));

    load_key_and_cert(&mut client_config).unwrap();
    //client_config.versions = vec![];
    let dns_name = DNSNameRef::try_from_ascii_str("www.google.com").unwrap();
    return ClientSession::new(&Arc::new(client_config), dns_name);
}

pub fn raw_message(channel: Channel, flags: i32, type_x: i32, data: &[u8], stream: &mut impl Write) -> io::Result<usize> {
    let total = raw_messsage_header(channel, flags, type_x, stream, data.len())?;
    let foo = stream.write(data)?;
    Ok(total + foo)
}

fn raw_messsage_header(channel: Channel, flags: i32, type_x: i32, stream: &mut impl Write, size: usize) -> io::Result<usize> {
    let total = 6 + size;
    let buffer: [u8; 6] = [
        channel.asInt(),
        flags as u8,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_message() {

        let mut mock = Mock::new();
        raw_message(Channel::CONTROL, 3, 1, &[0, 1, 0, 1], &mut mock).unwrap();
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
