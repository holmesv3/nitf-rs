//! Example to read a nitf and print metadata

use nitf_rs::ImageSegment;

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

    let mut nitf = nitf_rs::Nitf::default();
    log::info!("Created empty NITF successfully");
    
    nitf.file = Some(std::fs::File::create(out_path).unwrap());
    let mut img_header = ImageSegment::default();
    img_header.data_size = 512;
    img_header.header.nbands.val = 1;
    let mut img_header2 = ImageSegment::default();
    img_header2.data_size = 512;
    img_header2.header.nbands.val = 1;
    nitf.add_im(img_header);
    nitf.add_im(img_header2);

    let n_bytes = nitf.write_headers().unwrap();
    
    log::debug!("Wrote {n_bytes} bytes");
    log::info!("Successfully wrote NITF");

    let nitf_path = std::path::Path::new(&args[1]);
    log::info!("Found NITF at: {}", nitf_path.display());

    let nitf = nitf_rs::read_nitf(nitf_path).unwrap();
    log::info!("Read NITF successfully");

    log::info!("NITF metadata: {}", nitf);
}
