//! Example to read a nitf and print metadata

fn usage() {
    eprintln!("Example of writing an empty NITF file with all log output");
    eprintln!("Usage: cargo run --example write <OUT-PATH>");
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

    let mut nitf = nitf_rs::Nitf::default();
    log::info!("Created empty NITF successfully");

    let mut img_seg = nitf_rs::ImageSegment::default();
    img_seg.data_size = 128;
    let image_data = Vec::from_iter(0u8..img_seg.data_size as u8);

    nitf.add_im(img_seg);

    let mut file = std::fs::File::create(out_path).unwrap();
    let n_bytes = nitf.write_headers(&mut file).unwrap();
    nitf.image_segments[0]
        .write_data(&mut file, &image_data.as_slice())
        .unwrap();

    log::debug!("Wrote {n_bytes} bytes");
    log::info!("Successfully wrote NITF");
}
