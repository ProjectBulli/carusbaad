extern crate libusb;
extern crate webpki;

use std::time::Duration;
use libusb::{Device, DeviceDescriptor, Error};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

const VENDOR_GOOGLE: u16 = 0x18d1;
const PRODUCT_ID_MIN: u16 = 0x2D00;
const PRODUCT_ID_MAX: u16 = 0x2D05;

fn main() -> Result<(), Error> {
    println!("CarUSBAAd");
    let context = libusb::Context::new()?;
    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;

        if device_desc.vendor_id() == VENDOR_GOOGLE
            && device_desc.product_id() <= PRODUCT_ID_MAX
            && device_desc.product_id() >= PRODUCT_ID_MIN {
            open(device, device_desc)?;
        }
    }
    Ok(())
}

fn open(device: Device, _device_desc: DeviceDescriptor) -> Result<(), Error> {
    let version_package: [u8; 10] = [0, 3, 0, 6, 0, 1, 0, 1, 0, 1];
    let timeout = Duration::from_secs(1);
    let mut buffer: [u8; 12] = [0; 12];
    let mut handle = device.open()?;
    let configuration = device.active_config_descriptor()?;
    for interface in configuration.interfaces() {
        let number = interface.number();
        let descriptor = interface.descriptors();
        for d in descriptor {
            for e in d.endpoint_descriptors() {
                println!("Interface {} with Endpoint {} Direction {:?} Transfer {:?} UsageType {:?} Address {} SyncType {:?}",
                         number, e.number(), e.direction(), e.transfer_type(), e.usage_type(), e.address(), e.sync_type());
            }
        }
    }
    handle.claim_interface(0)?;
    handle.write_bulk(1, &version_package, timeout)?; //Direction::Out
    let count = handle.read_bulk(129, &mut buffer, timeout)?; //Direction::In
    //[0, 3, 0, 8, 0, 2, 0, 1, 0, 5, 0, 0]
    println!("{} {:?}", count, buffer);
    //writeBufferToFile(buffer);
    let mut config = rustls::ClientConfig::new();
    let rc_config = Arc::new(config);
    let example_com = webpki::DNSNameRef::try_from_ascii_str("example.com").unwrap();
    let mut client = rustls::ClientSession::new(&rc_config, example_com);

    Ok(())
}
