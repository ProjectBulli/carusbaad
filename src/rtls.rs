use rustls::{ClientConfig, ClientSession, Stream};
use libusb::{DeviceHandle, Error};
use webpki::DNSNameRef;

use crate::usbbulkstream::{USBBulkStream};
use crate::x509::{CERTIFICATE, PRIVATE_KEY};
use std::io::{BufReader, Read, Write};
use std::sync::Arc;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(20);

pub fn connect_rustls(socket: &mut USBBulkStream) -> Result<(), Error> {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut client_config = ClientConfig::new();

    client_config.enable_sni = false;
    client_config.enable_early_data = true;

//    client_config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    load_key_and_cert(&mut client_config).unwrap();
    //client_config.versions = vec![];
    let dns_name = DNSNameRef::try_from_ascii_str("www.google.com").unwrap();
    let mut client = ClientSession::new(&Arc::new(client_config), dns_name);

    let mut stream =  rustls::Stream::new(&mut client,  socket);

    println!("first contact");
    let mut plaintext = Vec::new();
    stream.read_to_end(&mut plaintext).unwrap();
    println!("read done");
    Ok(())
}

fn load_certs() -> Result<Vec<rustls::Certificate>, ()> {
    let mut reader = BufReader::new(CERTIFICATE.as_bytes());
    rustls::internal::pemfile::certs(&mut reader)
}

fn load_private_key() -> Result<rustls::PrivateKey, ()> {
    let mut reader = BufReader::new(PRIVATE_KEY.as_bytes());
    let keys = rustls::internal::pemfile::rsa_private_keys(&mut reader)?;
    Ok(keys[0].clone())
}

fn load_key_and_cert(config: &mut rustls::ClientConfig) -> Result<(), ()>{
    let certs = load_certs()?;
    let privkey = load_private_key()?;

    config.set_single_client_cert(certs, privkey).unwrap();
    Ok(())
}
