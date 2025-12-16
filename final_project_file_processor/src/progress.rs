#[derive(Debug)]
pub struct Progress {
    pub total_files: usize,
    pub processed_files: usize,
    pub failed_files: usize,
}

impl Default for Progress {
    fn default() -> Self {
        Progress {
            total_files: 0,
            processed_files: 0,
            failed_files: 0,
        }
    }
}
