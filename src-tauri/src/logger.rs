use std::{io::Write, path::PathBuf};

#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Error,
}

pub fn write_log(base_path: &PathBuf, log_level: LogLevel, message: &str) {
    let log_directory = base_path.join("logs");
    if !log_directory.try_exists().unwrap_or(false) {
        std::fs::create_dir_all(&log_directory).expect("Failed to create log directory");
    }

    let log_file_path = match log_level {
        LogLevel::Debug => log_directory.join("debug.log"),
        LogLevel::Error => log_directory.join("error.log"),
    };

    if !log_file_path.try_exists().unwrap_or(false) {
        std::fs::File::create(&log_file_path).expect("Failed to create log file");
    }

    let log_message = format!(
        "[{:?}] [{}] {}\n",
        log_level,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        message
    );

    let mut log_file = std::fs::OpenOptions::new()
        .append(true)
        .open(&log_file_path)
        .expect("cannot open file");

    log_file
        .write(log_message.as_bytes())
        .expect("cannot write to file");
}
