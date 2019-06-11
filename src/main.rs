#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate libusb;
extern crate webpki;
extern crate hex;

mod x509;
mod usbbulkstream;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use usbbulkstream::USBBulkStream;
use x509::{PRIVATE_KEY, CERTIFICATE};
use std::sync::Arc;
use std::time::Duration;
use std::io;
use libusb::{Device, DeviceDescriptor, Error, DeviceHandle};
use std::io::{Write, Read, BufReader};
use rustls::Stream;
use std::net::TcpStream;
use webpki_roots;
use native_tls::TlsConnector;

const VENDOR_GOOGLE: u16 = 0x18d1;
const PRODUCT_ID_MIN: u16 = 0x2D00;
const PRODUCT_ID_MAX: u16 = 0x2D05;

fn main() -> Result<(), Error> {
    println!("CarUSBAAd");
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

   // tls_check();
    //native_tls_check();
    println!("check done");
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
    let mut buffer: [u8; 12] = [0; 12];
    let version_package: [u8; 10] = [0, 3, 0, 6, 0, 1, 0, 1, 0, 1];
    let timeout = Duration::from_secs(10);
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
    handle.write_bulk(1, &version_package, timeout)?; //Direction::Out
    println!("waiting for handshake response");
   // let count = handle.read_bulk(129, &mut buffer, timeout)?; //Direction::In
    //[0, 3, 0, 8, 0, 2, 0, 1, 0, 5, 0, 0]
   // println!("read {} {:?}", count, buffer);
    //connectRusTLS(timeout, handle);
    connectNativeTLS(timeout, handle);

    Ok(())
}

fn connectRusTLS(timeout: Duration, mut handle: DeviceHandle) {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut config = rustls::ClientConfig::new();
    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
//load_key_and_cert(&mut config);
    let dns_name = webpki::DNSNameRef::try_from_ascii_str("www.google.com").unwrap();
    let mut client = rustls::ClientSession::new(&Arc::new(config), dns_name);
    let mut socket = USBBulkStream::new(129, 1, timeout, handle);
//TODO this must be the usb connection
    let mut stream = Stream::new(&mut client, &mut socket);
    println!("first contact");
    let read_result = stream.read(&mut buffer);
    match read_result {
        Err(E) => println!("Error reading {:?}", E),
        Ok(size) => println!("successful read {} bytes", size)
    }
    println!("read done");
//0 3 0 8 2 0 1 0 5
// println!("first read {}", hex::encode(buffer));
    stream.write("foo".as_bytes());
    println!("first write");
}

fn connectNativeTLS(timeout: Duration, mut handle: DeviceHandle) {
  //  let mut buffer: [u8; 1024] = [0; 1024];
    let mut socket = USBBulkStream::new(129, 1, timeout, handle);
 //   let mut tls = TlsConnector::builder().use_sni(false).build().unwrap();
 //   tls.connect("google.com", socket).unwrap();
    let tls = TlsConnector::new().unwrap();
    let mut s = tls.connect("google.com", socket);
    match &mut s {
        Ok(stream) => {
            let mut res = vec![];
            stream.read_to_end(&mut res);
            println!("{}", String::from_utf8_lossy(&res));
        }
        Err(_) => {}
    }
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

fn tls_check() {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut config = rustls::ClientConfig::new();
    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    //load_key_and_cert(&mut config);
    let dns_name = webpki::DNSNameRef::try_from_ascii_str("www.google.com").unwrap();
    let mut client = rustls::ClientSession::new(&Arc::new(config), dns_name);
    //let mut socket = USBBulkStream::new ( 129,  1, timeout, handle );//TODO this must be the usb connection
    let mut socket = TcpStream::connect("www.google.com:443").unwrap();
    let mut stream = Stream::new ( &mut client,  &mut socket);
    println!("first contact");
    stream.write(concat!("GET / HTTP/1.1\r\n",
                      "Host: google.com\r\n",
                      "Connection: close\r\n",
                      "Accept-Encoding: identity\r\n",
                      "\r\n")
        .as_bytes())
        .unwrap();
    let read_result = stream.read(&mut buffer);
}

fn native_tls_check() {
    let connector = TlsConnector::new().unwrap();

    let stream = TcpStream::connect("google.com:443").unwrap();
    let mut stream = connector.connect("google.com", stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}