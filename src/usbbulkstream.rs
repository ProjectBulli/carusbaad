use std::io::{Read, Write};
use std::time::Duration;
use libusb::DeviceHandle;
use std::io;

pub struct USBBulkStream<'l> {
    read_endpoint: u8,
    write_endpoint: u8,
    timeout: Duration,
    device_handle: DeviceHandle<'l>
}

impl<'l> USBBulkStream<'l> {
    pub fn new(read_endpoint: u8,
               write_endpoint: u8,
               timeout: Duration,
               device_handle: DeviceHandle<'l>) -> USBBulkStream {
        USBBulkStream {read_endpoint, write_endpoint, timeout, device_handle}
    }
}

impl <'l> Read for USBBulkStream<'l> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.device_handle.read_bulk(self.read_endpoint, buf, self.timeout).map_err(io_error_from_libusb_error)
    }
}

impl <'l> Write for USBBulkStream<'l> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.device_handle.write_bulk(self.write_endpoint, buf, self.timeout).map_err(io_error_from_libusb_error)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

//stupid workaroud, needs way better
fn io_error_from_libusb_error(error:libusb::Error) -> io::Error {
    io::Error::from_raw_os_error(-libusb_error_as_number(error))
}

fn libusb_error_as_number(error:libusb::Error) -> i32 {
    match error {
        libusb::Error::Success => 0,
        libusb::Error::Io => -101,
        libusb::Error::InvalidParam => -102,
        libusb::Error::Access => -103,
        libusb::Error::NoDevice => -104,
        libusb::Error::NotFound => -105,
        libusb::Error::Busy => -106,
        libusb::Error::Timeout => -107,
        libusb::Error::Overflow => -108,
        libusb::Error::Pipe => -109,
        libusb::Error::Interrupted => -110,
        libusb::Error::NoMem => -111,
        libusb::Error::NotSupported => -112,
        libusb::Error::Other => -113
    }
}