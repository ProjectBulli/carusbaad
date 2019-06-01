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
        self.device_handle.read_bulk(self.read_endpoint, buf, self.timeout).map_err(|_| io::Error::from_raw_os_error(-1))
    }
}

impl <'l> Write for USBBulkStream<'l> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.device_handle.write_bulk(self.write_endpoint, buf, self.timeout).map_err(|_| io::Error::from_raw_os_error(-1))
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}