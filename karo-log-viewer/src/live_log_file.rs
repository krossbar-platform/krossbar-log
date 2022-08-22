use std::{collections::VecDeque, path::PathBuf};

use chrono::NaiveDateTime;

use crate::{
    log_file_trait::{LogFile, ShiftDirection},
    rotated_log_file::RotatedLogFile,
};

pub struct LiveLogFile {
    inner: RotatedLogFile,
}

impl LiveLogFile {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            inner: RotatedLogFile::new(file_path, NaiveDateTime::from_timestamp(0, 0)),
        }
    }
}

impl LogFile for LiveLogFile {
    fn reset(&mut self) {
        self.inner.reset()
    }

    fn rev(&mut self) {
        self.inner.rev()
    }

    fn file_path(&self) -> PathBuf {
        self.inner.file_path()
    }

    fn lines(&self) -> &VecDeque<String> {
        self.inner.lines()
    }

    fn read_and_shift(
        &mut self,
        direction: ShiftDirection,
        window_size_lines: usize,
        shift_len: usize,
    ) -> (usize, usize) {
        self.inner
            .read_and_shift(direction, window_size_lines, shift_len)
    }
}
