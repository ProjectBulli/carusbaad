use crate::channel::Channel;
use crate::flags::Flags;
use crate::frame::{raw_messsage_header, FrameSender};
use rusb::{DeviceHandle, UsbContext};
use std::io;
use std::io::{Read, Write};
use std::time::Duration;

pub struct USBBulkStream<C:UsbContext> {
    read_endpoint: u8,
    write_endpoint: u8,
    timeout: Duration,
    device_handle: DeviceHandle<C>
}

impl<C:UsbContext> USBBulkStream<C> {
    pub fn new(read_endpoint: u8,
               write_endpoint: u8,
               timeout: Duration,
               device_handle: DeviceHandle<C>) -> USBBulkStream<C> {
        USBBulkStream {read_endpoint, write_endpoint, timeout, device_handle}
    }
}

impl <C:UsbContext> Read for USBBulkStream<C> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        println!("USB: read endpoint: {} length: {} timeout: {:?}", self.read_endpoint, buf.len(), self.timeout);
        let result = self.device_handle.read_bulk(self.read_endpoint, buf, self.timeout);
        println!("USB: read {:?}", result);
        result.map_err(io_error_from_libusb_error)
    }
}

impl <C:UsbContext> Write for USBBulkStream<C> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        println!("USB: write endpoint: {} length: {} timeout: {:?}", self.write_endpoint, buf.len(), self.timeout);
        let result = self.device_handle.write_bulk(self.write_endpoint, buf, self.timeout);
        println!("USB: write: {:?}", result);
        result.map_err(io_error_from_libusb_error)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

//stupid workaround, needs to be way better
pub fn io_error_from_libusb_error(error:rusb::Error) -> io::Error {
    io::Error::from_raw_os_error(-libusb_error_as_number(error))
}

//TODO use ErrorCode
fn libusb_error_as_number(error:rusb::Error) -> i32 {
    match error {
        rusb::Error::Io => -101,
        rusb::Error::InvalidParam => -102,
        rusb::Error::Access => -103,
        rusb::Error::NoDevice => -104,
        rusb::Error::NotFound => -105,
        rusb::Error::Busy => -106,
        rusb::Error::Timeout => -107,
        rusb::Error::Overflow => -108,
        rusb::Error::Pipe => -109,
        rusb::Error::Interrupted => -110,
        rusb::Error::NoMem => -111,
        rusb::Error::NotSupported => -112,
        rusb::Error::Other => -113,
        rusb::Error::BadDescriptor => -114,
    }
}


//TODO impl dyn Writer
impl <C:UsbContext> FrameSender for  USBBulkStream<C> {
    fn send(&mut self, channel: Channel, flags: Flags, type_x: i32, data: &[u8]) -> io::Result<usize> {
        let total = raw_messsage_header(channel, flags, type_x, self, data.len())?;
        let foo = self.write(data)?;
        Ok(total + foo)
    }
}
