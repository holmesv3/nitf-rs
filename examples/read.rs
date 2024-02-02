//! Example to read a nitf and print metadata
fn usage() {
    eprintln!("Example of reading a NITF file");
    eprintln!("Usage: cargo run --example read -- <NITF>");
    eprintln!(" Args: ");
    eprintln!("\tNITF: path to a NITF file");
}

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        log::error!("Must provide a NITF path");
        usage();
        return;
    }
    if args.len() > 2 {
        log::error!("Only one argument can be provided");
        usage();
        return;
    }

    let mut nitf_file = std::fs::File::open(&args[1]).unwrap();
    let nitf = nitf_rs::Nitf::from_reader(&mut nitf_file).unwrap();
    log::info!("Read NITF successfully");

    log::info!("NITF metadata: {nitf}");
}
