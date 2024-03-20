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

/// Example of reading a nitf
#[derive(Parser)]
struct Cli {
    /// Input file to read
    input: std::path::PathBuf,
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
