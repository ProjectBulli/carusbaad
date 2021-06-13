#![feature(in_band_lifetimes)]
extern crate chrono;
extern crate env_logger;
extern crate hex;
extern crate libusb;
extern crate log;
extern crate webpki;

use std::io::{Write, Read};
use std::io;
use std::time::Duration;

use chrono::Local;
use env_logger::Builder;
use libusb::{Device, DeviceDescriptor, Error, Context};
use log::LevelFilter;

mod protobuf;
mod x509;
mod usbbulkstream;

mod rtls;
use rtls::connect_rustls;

mod ntls;
use ntls::connect_ntls;

mod opensslx;
use opensslx::connect_openssl;
use crate::usbbulkstream::USBBulkStream;
use crate::rawmessage::{raw_message, VERSION_REQUEST};
use crate::channel::Channel;

mod rawmessage;
mod channel;

const VENDOR_GOOGLE: u16 = 0x18d1;
const PRODUCT_ID_MIN: u16 = 0x2D00;
const PRODUCT_ID_MAX: u16 = 0x2D05;
const TIMEOUT: Duration = Duration::from_secs(10);

fn open(device: Device, _device_desc: DeviceDescriptor) -> Result<(), Error> {
    println!("open {} {}", device.bus_number(), device.address());

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
    println!("claimed interface, write handshake data");

    let mut stream = USBBulkStream::new(129, 1, TIMEOUT, handle);
   stream.handshake().unwrap();
    connect_rustls( &mut stream )?;
    //connect_ntls( handle );
  //  connect_openssl( handle );
    Ok(())
}

impl <'l> USBBulkStream<'l> {
    fn handshake(&mut self) -> Result<usize, io::Error> {
        let mut buffer: [u8; 32] = [0; 32];
        //let version_package: [u8; 10] = [0, 3, 0, 6, 0, 1, 0, 1, 0, 1];
        //self.write(&version_package);

        raw_message(Channel::CONTROL, 3, 1, &VERSION_REQUEST, self);
        println!("waiting for handshake response");
        let count = self.read(&mut buffer);
        let size = count.unwrap();
        if size == 12 {
            //let response_excepted: [u8;12] = [0, 3, 0, 8, 0, 2, 0, 1, 0, 5, 0, 0];
            let response_excepted: [u8;12] =   [0, 3, 0, 8, 0, 2, 0, 1, 0, 6, 0, 0];
            if buffer.starts_with(&response_excepted) {
                println!("handshake done");
                Ok(size )
            } else {
                Err(io::Error::from_raw_os_error(-(size as i32)))
            }
        } else {
            Err(io::Error::from_raw_os_error(-(size as i32)))
        }
    }
}


fn main() ->  /*()*/  Result<(), Error>  {
    println!("CarUSBAAd");
 //   connect_openssl( );
  usb()
}

fn usb() -> Result<(), Error> {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Trace)
        .init();

    println!("check done");
    let context = Context::new()?;
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
