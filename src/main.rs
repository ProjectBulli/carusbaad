extern crate libusb;
extern crate webpki;
extern crate hex;

use std::sync::Arc;
use std::time::Duration;
use std::io;

use libusb::{Device, DeviceDescriptor, Error, DeviceHandle};
use std::io::{Write, Read, BufReader};
use rustls::Stream;

const VENDOR_GOOGLE: u16 = 0x18d1;
const PRODUCT_ID_MIN: u16 = 0x2D00;
const PRODUCT_ID_MAX: u16 = 0x2D05;

const CERTIFICATE: &'static str = "-----BEGIN CERTIFICATE-----\n\
MIIDKjCCAhICARswDQYJKoZIhvcNAQELBQAwWzELMAkGA1UEBhMCVVMxEzARBgNV\n\
BAgMCkNhbGlmb3JuaWExFjAUBgNVBAcMDU1vdW50YWluIFZpZXcxHzAdBgNVBAoM\n\
Fkdvb2dsZSBBdXRvbW90aXZlIExpbmswJhcRMTQwNzA0MDAwMDAwLTA3MDAXETQ1\n\
MDQyOTE0MjgzOC0wNzAwMFMxCzAJBgNVBAYTAkpQMQ4wDAYDVQQIDAVUb2t5bzER\n\
MA8GA1UEBwwISGFjaGlvamkxFDASBgNVBAoMC0pWQyBLZW53b29kMQswCQYDVQQL\n\
DAIwMTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAM911mNnUfx+WJtx\n\
uk06GO7kXRW/gXUVNQBkbAFZmVdVNvLoEQNthi2X8WCOwX6n6oMPxU2MGJnvicP3\n\
6kBqfHhfQ2Fvqlf7YjjhgBHh0lqKShVPxIvdatBjVQ76aym5H3GpkigLGkmeyiVo\n\
VO8oc3cJ1bO96wFRmk7kJbYcEjQyakODPDu4QgWUTwp1Z8Dn41ARMG5OFh6otITL\n\
XBzj9REkUPkxfS03dBXGr5/LIqvSsnxib1hJ47xnYJXROUsBy3e6T+fYZEEzZa7y\n\
7tFioHIQ8G/TziPmvFzmQpaWMGiYfoIgX8WoR3GD1diYW+wBaZTW+4SFUZJmRKgq\n\
TbMNFkMCAwEAATANBgkqhkiG9w0BAQsFAAOCAQEAsGdH5VFn78WsBElMXaMziqFC\n\
zmilkvr85/QpGCIztI0FdF6xyMBJk/gYs2thwvF+tCCpXoO8mjgJuvJZlwr6fHzK\n\
Ox5hNUb06AeMtsUzUfFjSZXKrSR+XmclVd+Z6/ie33VhGePOPTKYmJ/PPfTT9wvT\n\
93qswcxhA+oX5yqLbU3uDPF1ZnJaEeD/YN45K/4eEA4/0SDXaWW14OScdS2LV0Bc\n\
YmsbkPVNYZn37FlY7e2Z4FUphh0A7yME2Eh/e57QxWrJ1wubdzGnX8mrABc67ADU\n\
U5r9tlTRqMs7FGOk6QS2Cxp4pqeVQsrPts4OEwyPUyb3LfFNo3+sP111D9zEow==\n\
-----END CERTIFICATE-----\n";

const PRIVATE_KEY: &'static str = "-----BEGIN RSA PRIVATE KEY-----\n\
MIIEowIBAAKCAQEAz3XWY2dR/H5Ym3G6TToY7uRdFb+BdRU1AGRsAVmZV1U28ugR\n\
A22GLZfxYI7Bfqfqgw/FTYwYme+Jw/fqQGp8eF9DYW+qV/tiOOGAEeHSWopKFU/E\n\
i91q0GNVDvprKbkfcamSKAsaSZ7KJWhU7yhzdwnVs73rAVGaTuQlthwSNDJqQ4M8\n\
O7hCBZRPCnVnwOfjUBEwbk4WHqi0hMtcHOP1ESRQ+TF9LTd0Fcavn8siq9KyfGJv\n\
WEnjvGdgldE5SwHLd7pP59hkQTNlrvLu0WKgchDwb9POI+a8XOZClpYwaJh+giBf\n\
xahHcYPV2Jhb7AFplNb7hIVRkmZEqCpNsw0WQwIDAQABAoIBAB2u7ZLheKCY71Km\n\
bhKYqnKb6BmxgfNfqmq4858p07/kKG2O+Mg1xooFgHrhUhwuKGbCPee/kNGNrXeF\n\
pFW9JrwOXVS2pnfaNw6ObUWhuvhLaxgrhqLAdoUEgWoYOHcKzs3zhj8Gf6di+edq\n\
SyTA8+xnUtVZ6iMRKvP4vtCUqaIgBnXdmQbGINP+/4Qhb5R7XzMt/xPe6uMyAIyC\n\
y5Fm9HnvekaepaeFEf3bh4NV1iN/R8px6cFc6ELYxIZc/4Xbm91WGqSdB0iSriaZ\n\
TjgrmaFjSO40tkCaxI9N6DGzJpmpnMn07ifhl2VjnGOYwtyuh6MKEnyLqTrTg9x0\n\
i3mMwskCgYEA9IyljPRerXxHUAJt+cKOayuXyNt80q9PIcGbyRNvn7qIY6tr5ut+\n\
ZbaFgfgHdSJ/4nICRq02HpeDJ8oj9BmhTAhcX6c1irH5ICjRlt40qbPwemIcpybt\n\
mb+DoNYbI8O4dUNGH9IPfGK8dRpOok2m+ftfk94GmykWbZF5CnOKIp8CgYEA2Syc\n\
5xlKB5Qk2ZkwXIzxbzozSfunHhWWdg4lAbyInwa6Y5GB35UNdNWI8TAKZsN2fKvX\n\
RFgCjbPreUbREJaM3oZ92o5X4nFxgjvAE1tyRqcPVbdKbYZgtcqqJX06sW/g3r/3\n\
RH0XPj2SgJIHew9sMzjGWDViMHXLmntI8rVA7d0CgYBOr36JFwvrqERN0ypNpbMr\n\
epBRGYZVSAEfLGuSzEUrUNqXr019tKIr2gmlIwhLQTmCxApFcXArcbbKs7jTzvde\n\
PoZyZJvOr6soFNozP/YT8Ijc5/quMdFbmgqhUqLS5CPS3z2N+YnwDNj0mO1aPcAP\n\
STmcm2DmxdaolJksqrZ0owKBgQCD0KJDWoQmaXKcaHCEHEAGhMrQot/iULQMX7Vy\n\
gl5iN5E2EgFEFZIfUeRWkBQgH49xSFPWdZzHKWdJKwSGDvrdrcABwdfx520/4MhK\n\
d3y7CXczTZbtN1zHuoTfUE0pmYBhcx7AATT0YCblxrynosrHpDQvIefBBh5YW3AB\n\
cKZCOQKBgEM/ixzI/OVSZ0Py2g+XV8+uGQyC5XjQ6cxkVTX3Gs0ZXbemgUOnX8co\n\
eCXS4VrhEf4/HYMWP7GB5MFUOEVtlLiLM05ruUL7CrphdfgayDXVcTPfk75lLhmu\n\
KAwp3tIHPoJOQiKNQ3/qks5km/9dujUGU2ARiU3qmxLMdgegFz8e\n\
-----END RSA PRIVATE KEY-----\n";

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
    let mut socket = USBBulkStream { read_endpoint: 129, write_endpoint: 1, timeout, device_handle: handle };//TODO this must be the usb connection
    let mut s = Stream::new ( &mut client,  &mut socket);
    println!("first contact");
    s.read(&mut buffer);
    println!("first read {}", hex::encode(buffer));
    s.write("foo".as_bytes());
    println!("first write");

     Ok(())
}

struct USBBulkStream<'l> {
    read_endpoint: u8,
    write_endpoint: u8,
    timeout: Duration,
    device_handle: DeviceHandle<'l>
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