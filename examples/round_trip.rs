//! Example to read a nitf and print metadata

use nitf_rs::{headers::image_hdr::Band, *};

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
    let mut img_segment = ImageSegment::default();
    let band = Band::default();

    img_segment.data_size = 512;
    img_segment.header.nbands.val = 1;
    img_segment.header.bands.push(band.clone());
    nitf.add_im(img_segment);

    let ghx_segment = GraphicSegment::default();
    let txt_segment = TextSegment::default();
    let dex_segment = DataExtensionSegment::default();
    let rex_segment = ReservedExtensionSegment::default();

    nitf.add_sy(ghx_segment);
    nitf.add_te(txt_segment);
    nitf.add_de(dex_segment);
    nitf.add_re(rex_segment);

    let n_bytes = nitf.write_headers().unwrap();

    log::debug!("Wrote {n_bytes} bytes");
    log::info!("Successfully wrote NITF");

    let nitf_path = std::path::Path::new(&args[1]);
    let nitf_file = std::fs::File::open(nitf_path).unwrap();
    log::info!("Found NITF at: {}", nitf_path.display());

    let nitf = nitf_rs::read_nitf(nitf_file).unwrap();
    log::info!("Read NITF successfully");

    log::info!("NITF metadata: {}", nitf);
}
