use clap::{Parser, ValueEnum};
use log::LevelFilter;

#[derive(ValueEnum, Clone)]
enum ArgLevel {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}

impl From<ArgLevel> for LevelFilter {
    fn from(value: ArgLevel) -> Self {
        match value {
            ArgLevel::Off => Self::Off,
            ArgLevel::Error => Self::Error,
            ArgLevel::Warn => Self::Warn,
            ArgLevel::Info => Self::Info,
            ArgLevel::Debug => Self::Debug,
            ArgLevel::Trace => Self::Trace,
        }
    }
}

/// Example of writing a nitf file
#[derive(Parser)]
struct Cli {
    /// Ouput fold to write `write_example.nitf`
    #[arg(short, long, default_value = "./")]
    output: std::path::PathBuf,
    /// Log level
    #[arg(long, default_value = "warn")]
    level: ArgLevel,
}

fn main() {
    let args = Cli::parse();
    simple_logger::SimpleLogger::new()
        .with_level(args.level.into())
        .init()
        .unwrap();

    // Create the output path and file
    match args
        .output
        .try_exists()
        .expect("No read access to output directory")
    {
        true => Ok(()),
        false => std::fs::create_dir_all(&args.output),
    }
    .expect("Could not create output directory");

    let out_path = args.output.join("write_example.nitf");
    let mut file = std::fs::File::create(out_path).unwrap();

    // Create and write a nitf
    let mut nitf = nitf_rs::Nitf::default();

    let img_seg = nitf_rs::ImageSegment {
        data_size: 2 ^ 10,
        ..Default::default()
    };
    let image_data = vec![0u8; img_seg.data_size as usize];

    nitf.add_im(img_seg);

    let n_bytes = nitf.write_headers(&mut file).unwrap();
    nitf.image_segments[0]
        .write_data(&mut file, image_data.as_slice())
        .unwrap();

    log::debug!("Wrote {n_bytes} bytes");
    log::info!("Successfully wrote NITF");
}
