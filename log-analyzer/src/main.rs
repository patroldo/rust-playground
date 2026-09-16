use clap::Parser;
use log_analyzer::ErrorReadingFile;

use log_analyzer::log_parsing::{log_entry::LogEntry, log_level::LogLevel};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn count_logs_per_type(logs: &[LogEntry], log_level: LogLevel) {
    println!(
        "{}: {:?}",
        log_level,
        logs.iter().filter(|log| log.log_level == log_level).count()
    );
}

fn main() -> Result<(), ErrorReadingFile> {
    let args = Cli::parse();
    let (logs, errors) = log_analyzer::parse_logs_from_file(args.path)?;
    eprintln!("Unparsable logs: {}", errors.len());
    count_logs_per_type(&logs, LogLevel::InfoL);
    count_logs_per_type(&logs, LogLevel::WarnL);
    count_logs_per_type(&logs, LogLevel::ErrorL);
    Ok(())
}
