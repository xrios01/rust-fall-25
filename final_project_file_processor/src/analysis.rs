use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct ProcessingError {
    pub message: String,
}

#[derive(Debug)]
pub struct FileStats {
    pub word_count: usize,
    pub line_count: usize,
    pub char_frequencies: HashMap<char, usize>,
    pub size_bytes: u64,
}

#[derive(Debug)]
pub struct FileAnalysis {
    pub filename: String,
    pub stats: FileStats,
    pub errors: Vec<ProcessingError>,
    pub processing_time: Duration,
}

/// Trait for an extensible analyzer system.
/// New analyzers can be added by implementing this trait.
pub trait Analyzer {
    fn name(&self) -> &str;

    fn analyze(&self, contents: &str, size_bytes: u64, stats: &mut FileStats);
}

/// Analyzer that computes word and line counts.
pub struct WordLineAnalyzer;

impl Analyzer for WordLineAnalyzer {
    fn name(&self) -> &str {
        "word_line"
    }

    fn analyze(&self, contents: &str, _size_bytes: u64, stats: &mut FileStats) {
        stats.line_count = contents.lines().count();
        stats.word_count = contents.split_whitespace().count();
    }
}

/// Analyzer that builds a character frequency table.
pub struct CharFrequencyAnalyzer;

impl Analyzer for CharFrequencyAnalyzer {
    fn name(&self) -> &str {
        "char_frequency"
    }

    fn analyze(&self, contents: &str, _size_bytes: u64, stats: &mut FileStats) {
        let mut freqs = HashMap::new();
        for ch in contents.chars() {
            *freqs.entry(ch).or_insert(0) += 1;
        }
        stats.char_frequencies = freqs;
    }
}

pub fn analyze_file(path: &Path) -> FileAnalysis {
    let start_time = Instant::now();
    let mut errors = Vec::new();

    let filename = path.to_string_lossy().to_string();

    // File size
    let size_bytes = match fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(e) => {
            errors.push(ProcessingError {
                message: format!("Failed to get metadata: {}", e),
            });
            0
        }
    };

    // Read file contents
    let mut contents = String::new();
    if let Err(e) = File::open(path).and_then(|mut f| f.read_to_string(&mut contents)) {
        errors.push(ProcessingError {
            message: format!("Failed to read file: {}", e),
        });
    }

    // Initial stats with default values; analyzers will fill the rest.
    let mut stats = FileStats {
        word_count: 0,
        line_count: 0,
        char_frequencies: HashMap::new(),
        size_bytes,
    };

    // Extensible analyzer system: just push new analyzers here.
    let analyzers: Vec<Box<dyn Analyzer>> = vec![
        Box::new(WordLineAnalyzer),
        Box::new(CharFrequencyAnalyzer),
    ];

    for analyzer in &analyzers {
        analyzer.analyze(&contents, size_bytes, &mut stats);
    }

    let processing_time = start_time.elapsed();

    FileAnalysis {
        filename,
        stats,
        errors,
        processing_time,
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    #[test]
    fn analyze_file_counts_words_and_lines_correctly() {
        // Create a temporary file with known content
        let mut path = env::temp_dir();
        path.push("fp_test_file.txt");

        {
            let mut f = File::create(&path).unwrap();
            // 2 lines, 6 words total
            writeln!(f, "hello world").unwrap();        // 2 words
            writeln!(f, "this is a test").unwrap();     // 4 words
        }

        let analysis = analyze_file(&PathBuf::from(&path));

        assert_eq!(analysis.stats.line_count, 2);
        assert_eq!(analysis.stats.word_count, 6);
        assert!(analysis.stats.size_bytes > 0);
        assert!(analysis.errors.is_empty());
    }

    #[test]
    fn analyze_file_reports_error_for_missing_file() {
        let bogus = Path::new("this_file_should_not_exist_123456.txt");
        let analysis = analyze_file(bogus);

        assert!(!analysis.errors.is_empty());
    }

    #[test]
    //#[ignore] // run manually: cargo test perf_large_text -- --ignored
    fn perf_large_text() {
        // Simulate a large text file (~1 million characters)
        let contents = "hello world\n".repeat(100_000);

        let start = std::time::Instant::now();

        // Do similar work to what our analyzers do
        let line_count = contents.lines().count();
        let word_count = contents.split_whitespace().count();

        let mut char_frequencies = std::collections::HashMap::new();
        for ch in contents.chars() {
            *char_frequencies.entry(ch).or_insert(0) += 1;
        }

        let elapsed = start.elapsed();

        // Just assert it's not absurdly slow (very generous bound)
        assert!(
            elapsed.as_secs() < 2,
            "perf_large_text too slow: {:?}, lines={}, words={}",
            elapsed,
            line_count,
            word_count
        );
    }
}
