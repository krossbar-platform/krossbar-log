use crate::{args, LogEvent};

use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::rotator::Rotator;

pub struct Writer {
    log_file: Option<File>,
    log_location: PathBuf,
    rotator: Rotator,
    current_file_num_bytes: u64,
    max_file_len: u64,
}

impl Writer {
    pub fn new(args: &args::Args) -> Self {
        println!("Log file location: {:?}", args.log_location);

        let mut this = Self {
            log_file: None,
            log_location: PathBuf::from(&args.log_location),
            rotator: Rotator::new(args.keep_num_files, PathBuf::from(&args.log_location)),
            current_file_num_bytes: 0,
            max_file_len: args.num_bytes_rotate,
        };

        this.open_log_file();
        this
    }

    fn open_log_file(&mut self) {
        self.log_file = Some(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(&self.log_location)
                .expect(&format!(
                    "Failed to open log file at {:?}",
                    &self.log_location
                )),
        );
    }

    fn close_log_file(&mut self) {
        self.log_file = None;
    }

    pub fn log_message(&mut self, message: LogEvent) -> Option<String> {
        let log_line = format!(
            "<{}> {}#{} [{}] {} > {}\n",
            message.message.timestamp.format("%d-%m-%Y %H:%M:%S%.3f"),
            message.service_name,
            message.pid,
            message.message.level,
            message.message.target,
            message.message.message
        );

        // New current log len
        self.current_file_num_bytes += log_line.len() as u64;

        match self.log_file {
            Some(ref mut log_file) => {
                if let Err(err) = log_file.write_all(log_line.as_bytes()) {
                    eprintln!("Failed to write log message: {}", err.to_string())
                }
            }
            _ => {
                eprintln!("Failed to write log message. Log file is closed");
            }
        }

        self.check_rotate()
    }

    fn check_rotate(&mut self) -> Option<String> {
        if self.current_file_num_bytes < self.max_file_len {
            return None;
        }

        self.close_log_file();

        let rotate_file_path = self.rotator.rotate();
        self.current_file_num_bytes = 0;

        self.open_log_file();

        Some(rotate_file_path)
    }
}
