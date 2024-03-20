use clap::Parser;
use log::LevelFilter;

/// Example of reading a nitf
#[derive(Parser)]
struct Cli {
    /// Input file to read
    input: std::path::PathBuf,
    /// Log level
    #[arg(long, default_value = "warn")]
    level: LevelFilter,
}

fn main() {
    let args = Cli::parse();
    simple_logger::SimpleLogger::new()
        .with_level(args.level)
        .init()
        .unwrap();

    let mut nitf_file = std::fs::File::open(&args.input).unwrap();
    let nitf = match nitf_rs::Nitf::from_reader(&mut nitf_file) {
        Ok(n) => {
            log::info!("Read NITF successfully");
            n
        }
        Err(e) => {
            log::error!("{e}");
            return;
        }
    };
    log::debug!("Nitf metadata: {nitf}");
}
