use openssl::ssl::{SslMethod, SslConnector};
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::TIMEOUT;
use crate::usbbulkstream::USBBulkStream;
use libusb::DeviceHandle;

pub fn connect_openssl(handle: DeviceHandle) -> () {
/*    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = USBBulkStream::new(129, 1, TIMEOUT, handle);
    //let stream = TcpStream::connect("www.google.com:443").unwrap();
    let mut stream = connector.connect("www.google.com", stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));*/
}
