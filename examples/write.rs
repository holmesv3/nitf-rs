//! Example to read a nitf and print metadata

use std::fs::File;

fn usage() {
    eprintln!("Example of writing a NITF file");
    eprintln!("Usage: cargo run --example write -- <OUT>");
    eprintln!(" Args: ");
    eprintln!("\tOUT: path to write default NITF file");
}

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        log::error!("Must provide an output path");
        usage();
        return;
    }
    if args.len() > 2 {
        log::error!("Only one argument can be provided");
        usage();
        return;
    }

    let out_path = std::path::Path::new(&args[1]);
    log::info!("Found OUT path: {}", out_path.display());
    let mut nitf = nitf_rs::Nitf::default();

    log::info!("Created NITF successfully");
    nitf.file = Some(File::create(out_path).unwrap());
    nitf.write_headers().unwrap();
    log::info!("Successfully wrote NITF");
}
