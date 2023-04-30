use std::path::Path;

use nitf_rs;

fn main() {
    let path = Path::new("C:/Users/vaugh/Dev/data/sicd/2023-02-08-11-51-12_UMBRA-04_SICD.nitf");
    let nitf = nitf_rs::read_nitf(Some(path)).unwrap();
    println!("{}", nitf);
    dbg!("{}", nitf);
}