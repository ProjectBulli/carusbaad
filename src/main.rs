use rusb::{Device, DeviceList, Error, GlobalContext, UsbContext};
use std::io;
use std::io::{Read, Write};
use std::time::Duration;

use crate::rawmessage::{status_ok, version_request};
use crate::usbbulkstream::USBBulkStream;

mod x509;
mod usbbulkstream;

mod rawmessage;
mod channel;
mod flags;
mod frame;

const VENDOR_GOOGLE: u16 = 0x18d1;
const PRODUCT_ID_MIN: u16 = 0x2D00;
const PRODUCT_ID_MAX: u16 = 0x2D05;
const TIMEOUT: Duration = Duration::from_secs(10);

fn open(device: Device<GlobalContext>) -> Result<(), Error> {
    println!("open {} {}", device.bus_number(), device.address());

    let handle = device.open()?;
    let configuration = device.active_config_descriptor()?;
    for interface in configuration.interfaces() {
        let interface_number = interface.number();
        let descriptor = interface.descriptors();
        for d in descriptor {
            for e in d.endpoint_descriptors() {
                println!("Interface {} with Endpoint {} Direction {:?} Transfer {:?} UsageType {:?} Address {} SyncType {:?}",
                         interface_number, e.number(), e.direction(), e.transfer_type(), e.usage_type(), e.address(), e.sync_type());
            }
        }
    }
    handle.claim_interface(0)?;
    println!("claimed interface, write handshake data");


    Ok(())
}

impl <C:UsbContext> USBBulkStream<C> {
    fn handshake(&mut self) -> Result<usize, io::Error> {
        let mut buffer: [u8; 32] = [0; 32];
        //let version_package: [u8; 10] = [0, 3, 0, 6, 0, 1, 0, 1, 0, 1];
        //self.write(&version_package);

        version_request(self)?;
        println!("waiting for handshake response");
        let count = self.read(&mut buffer)?;
        if count == 12 {
            //let response_excepted: [u8;12] = [0, 3, 0, 8, 0, 2, 0, 1, 0, 5, 0, 0];
            let response_excepted: [u8;12] =   [0, 3, 0, 8, 0, 2, 0, 1, 0, 6, 0, 0];
            if buffer.starts_with(&response_excepted) {
                println!("handshake done");
              //  tls_handshake(&mut create_tlsclient(), self);
                println!("tls done");
                status_ok(self)?;
                Ok(count )
            } else {
                Err(io::Error::from_raw_os_error(-(count as i32)))
            }
        } else {
            Err(io::Error::from_raw_os_error(-(count as i32)))
        }
    }
}


fn main() ->  Result<(), Error>  {
    println!("CarUSBAAd");
  usb()
}

fn usb() -> Result<(), Error> {
    for device in DeviceList::new()?.iter() {
        let device_desc = device.device_descriptor()?;
        println!("scan {} {} {}:{}", device.bus_number(), device.address(), device_desc.vendor_id(), device_desc.product_id());

        if device_desc.vendor_id() == VENDOR_GOOGLE
            && device_desc.product_id() <= PRODUCT_ID_MAX
            && device_desc.product_id() >= PRODUCT_ID_MIN {
            open(device)?;
        }
    }
    Ok(())
}
