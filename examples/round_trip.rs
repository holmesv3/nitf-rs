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

    let mut nitf_out = nitf_rs::Nitf::default();
    log::info!("Created empty NITF successfully");
    
    let mut file = std::fs::File::create(out_path).unwrap();
    let mut img_segment = ImageSegment::default();
    let band = Band::default();
    let data = Vec::from_iter(0u8..8);

    img_segment.data_size = data.len() as u64;
    img_segment.header.nbands.val = 1;
    img_segment.header.bands.push(band);
    nitf_out.add_im(img_segment);

    let ghx_segment = GraphicSegment::default();
    let txt_segment = TextSegment::default();
    let dex_segment = DataExtensionSegment::default();
    let rex_segment = ReservedExtensionSegment::default();

    nitf_out.add_sy(ghx_segment);
    nitf_out.add_te(txt_segment);
    nitf_out.add_de(dex_segment);
    nitf_out.add_re(rex_segment);

    let mut n_bytes = nitf_out.write_headers(&mut file).unwrap();
    n_bytes += nitf_out.image_segments[0].write_data(&mut file, data.as_slice()).unwrap();

    log::debug!("Wrote {n_bytes} bytes");
    log::info!("Successfully wrote NITF");

    let mut nitf_file = std::fs::File::open(out_path).unwrap();

    let nitf_in = nitf_rs::read_nitf(&mut nitf_file).unwrap();
    let im_data_read = nitf_in.image_segments[0].read_data(&mut nitf_file).unwrap();
    log::info!("Read NITF successfully");

    log::info!("NITF metadata: {}", nitf_in);
    assert_eq!(nitf_out, nitf_in, "Verifying the metadata we wrote and read are equivalent... ");
    assert_eq!(data, im_data_read[..], "Verifying the data we wrote and read are equivalent... ");
}
