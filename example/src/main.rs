#[macro_use]
extern crate into_cdrs_bytes_derive;
extern crate cdrs;

use cdrs::types::value::{Value, Bytes};
use cdrs::{IntoCDRSBytes, IntoBytes};

#[derive(Debug, IntoCDRSBytes)]
struct Udt {
    pub number: i32,
    pub number_16: i16,
    pub number_8: N,
}

#[derive(Debug, IntoCDRSBytes)]
struct N {
    pub n: i16,
}

fn main() {
    let udt = Udt {
        number: 12,
        number_16: 256,
        number_8: N { n: 100 },
    };
    let val: Value = udt.into_cdrs_bytes().into();
    println!("values {:?}", val);
}
