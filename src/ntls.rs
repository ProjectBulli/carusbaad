use libusb::{DeviceHandle, Error};

use native_tls::{TlsConnector, TlsStream, HandshakeError, Certificate, Identity};
use std::io::{Read, Write, BufReader};
use std::net::TcpStream;
use std::time::Duration;

use crate::usbbulkstream::{USBBulkStream};
use native_tls::Protocol::Tlsv12;
use crate::x509::{CERTIFICATE, PRIVATE_KEY};

const TIMEOUT: Duration = Duration::from_secs(10);

pub fn connect_ntls(handle: DeviceHandle) -> Result<(), ()> {
    let mut reader = BufReader::new(CERTIFICATE.as_bytes());

    let connector = TlsConnector::builder()
        .use_sni(false)
        .min_protocol_version(Some(Tlsv12))
        //.add_root_certificate(Certificate::from_pem(CERTIFICATE.as_bytes()).unwrap())
       // .identity(Identity::from_pkcs12(PRIVATE_KEY.as_bytes()), "".)
        .danger_accept_invalid_hostnames(true)
        .danger_accept_invalid_certs(true)

        .build().unwrap();

    let stream = USBBulkStream::new(129, 1, TIMEOUT, handle);
    let c = connector.connect("", stream);

    match c {
        Ok(s) => {
            
            println!("connected");
        },
        _ => println!("not connected ")
    }

    Ok(())
}
