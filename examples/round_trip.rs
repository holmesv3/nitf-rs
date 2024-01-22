//! Example to read a nitf and print metadata
fn usage() {
    eprintln!("Example of writing and reading a NITF file");
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

    let nitf = nitf_rs::Nitf::default();
    log::info!("Created empty NITF successfully");

    let n_bytes = nitf.write(out_path).unwrap();
    log::debug!("Wrote {n_bytes} bytes");
    log::info!("Successfully wrote NITF");

    let nitf_path = std::path::Path::new(&args[1]);
    log::info!("Found NITF at: {}", nitf_path.display());

    let nitf = nitf_rs::read_nitf(nitf_path).unwrap();
    log::info!("Read NITF successfully");

    log::info!("NITF metadata: {}", nitf);
}
