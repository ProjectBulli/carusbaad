extern crate libusb;
extern crate webpki;
extern crate hex;

mod x509;
mod usbbulkstream;

use usbbulkstream::USBBulkStream;

use x509::{PRIVATE_KEY, CERTIFICATE};
use std::sync::Arc;
use std::time::Duration;
use std::io;

use libusb::{Device, DeviceDescriptor, Error, DeviceHandle};
use std::io::{Write, Read, BufReader};
use rustls::Stream;

const VENDOR_GOOGLE: u16 = 0x18d1;
const PRODUCT_ID_MIN: u16 = 0x2D00;
const PRODUCT_ID_MAX: u16 = 0x2D05;

fn main() -> Result<(), Error> {
    println!("CarUSBAAd");
    let context = libusb::Context::new()?;
    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;
        println!("scan {} {} {}:{}", device.bus_number(), device.address(), device_desc.vendor_id(), device_desc.product_id());

        if device_desc.vendor_id() == VENDOR_GOOGLE
            && device_desc.product_id() <= PRODUCT_ID_MAX
            && device_desc.product_id() >= PRODUCT_ID_MIN {
            open(device, device_desc)?;
        }
    }
    Ok(())
}

fn open(device: Device, _device_desc: DeviceDescriptor) -> Result<(), Error> {
    println!("open {} {}", device.bus_number(), device.address());
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
    let mut config = rustls::ClientConfig::new();
    load_key_and_cert(&mut config);
    let rc_config = Arc::new(config);
    let example_com = webpki::DNSNameRef::try_from_ascii_str("example.com").unwrap();
    let mut client = rustls::ClientSession::new(&rc_config, example_com);
    let mut socket = USBBulkStream::new ( 129,  1, timeout, handle );//TODO this must be the usb connection
    let mut s = Stream::new ( &mut client,  &mut socket);
    println!("first contact");
    s.read(&mut buffer);
    println!("first read {}", hex::encode(buffer));
    s.write("foo".as_bytes());
    println!("first write");

    Ok(())
}

fn load_certs() -> Vec<rustls::Certificate> {
    let mut reader = BufReader::new(CERTIFICATE.as_bytes());
    rustls::internal::pemfile::certs(&mut reader).unwrap()
}

fn load_private_key() -> rustls::PrivateKey {
    let mut reader = BufReader::new(PRIVATE_KEY.as_bytes());
    let keys = rustls::internal::pemfile::rsa_private_keys(&mut reader).unwrap();
    keys[0].clone()
}

fn load_key_and_cert(config: &mut rustls::ClientConfig) {
    let certs = load_certs();
    let privkey = load_private_key();

    config.set_single_client_cert(certs, privkey);
}

// Include the `items` module, which is generated from items.proto.
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
}

pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}