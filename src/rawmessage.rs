use std::io;
use std::io::Write;

use bytebuffer::ByteBuffer;
use rustls::ClientConnection;

use crate::channel::Channel;
use crate::flags::Flags;
use crate::frame::FrameSender;

/*
 see https://github.com/harry1453/android-auto-headunit/blob/master/app/src/main/java/info/anodsplace/headunit/aap/protocol/messages/Messages.kt
 https://github.com/anod/headunit/blob/master/app/src/main/java/info/anodsplace/headunit/aap/AapTransport.kt
*/

pub fn version_request( stream: &mut impl FrameSender) -> io::Result<usize> {
    return stream.send(Channel::CONTROL, Flags::FIRST_FRAME | Flags::LAST_FRAME, 1, &[0, 1, 0, 1]);
}

pub fn status_ok( stream: &mut impl FrameSender) -> io::Result<usize> {
    return stream.send(Channel::CONTROL, Flags::FIRST_FRAME | Flags::LAST_FRAME, 4, &[8, 0]);
}

pub fn tls_handshake(tls_session: &mut ClientConnection, stream: &mut impl FrameSender)  -> io::Result<usize> {
    let buffer = &mut ByteBuffer::new();
    let mut tls_stream =  rustls::Stream::new(tls_session,  buffer);
    tls_stream.flush()?;
    let foo = buffer.as_bytes();
    stream.send(Channel::CONTROL, Flags::FIRST_FRAME | Flags::LAST_FRAME, 3, foo)
}

