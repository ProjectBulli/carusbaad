extern crate prost_build;

use std::env;

fn main() -> Result<(), std::io::Error> {
     prost_build::compile_protos(&["protobuf/ServiceDiscoveryRequestMessage.proto"],
                                &["protobuf/"])?;

    if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();

        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }

    Ok(())
}



