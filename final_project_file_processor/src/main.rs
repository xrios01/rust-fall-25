mod thread_pool;
mod analysis;
mod progress;

use crate::thread_pool::ThreadPool;
use crate::analysis::{FileAnalysis, analyze_file};
use crate::progress::Progress;

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Recursively collect all files under a directory
fn collect_files_from_dir(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_file() {
        out.push(dir.to_path_buf());
        return Ok(());
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_files_from_dir(&path, out)?;
            } else if path.is_file() {
                out.push(path);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <num_threads> <dir1> [dir2 ...]", args[0]);
        std::process::exit(1);
    }

    let num_threads: usize = args[1].parse().unwrap_or(4);
    let dir_args = &args[2..];

    // Collect all files from given directories
    let mut files: Vec<PathBuf> = Vec::new();
    for dir_str in dir_args {
        let path = Path::new(dir_str);
        if let Err(e) = collect_files_from_dir(path, &mut files) {
            eprintln!("Error scanning {}: {}", dir_str, e);
        }
    }

    if files.is_empty() {
        eprintln!("No files found in given directories.");
        return;
    }

    println!(
        "Found {} files. Using {} threads.",
        files.len(),
        num_threads
    );

    // Shared progress
    let progress = Arc::new(Mutex::new(Progress {
        total_files: files.len(),
        processed_files: 0,
        failed_files: 0,
    }));

    // Channel to receive per-file analysis
    let (results_tx, results_rx) = mpsc::channel::<FileAnalysis>();

    let pool = ThreadPool::new(num_threads);

    // Progress reporter thread
    let progress_for_reporter = Arc::clone(&progress);
    let reporter_handle = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        let p = progress_for_reporter.lock().unwrap();
        println!(
            "Progress: {}/{} processed (failed: {})",
            p.processed_files, p.total_files, p.failed_files
        );
        if p.processed_files >= p.total_files {
            break;
        }
    });

    // Submit jobs
    for path in files {
        let tx = results_tx.clone();
        let progress = Arc::clone(&progress);

        pool.execute(move || {
            let analysis = analyze_file(&path);

            // Update progress
            {
                let mut p = progress.lock().unwrap();
                p.processed_files += 1;
                if !analysis.errors.is_empty() {
                    p.failed_files += 1;
                }
            }

            // Send result back to main thread
            let _ = tx.send(analysis);
        });
    }

    // Drop the extra sender in main so the channel closes when worker are done
    drop(results_tx);

    // Collect results
    let mut all_results: Vec<FileAnalysis> = Vec::new();
    for analysis in results_rx {
        all_results.push(analysis);
    }

    // Wait for reporter thread to finish
    let _ = reporter_handle.join();

    // Print summary
    println!("\n=== Processing Summary ===");
    for res in &all_results {
        println!("File: {}", res.filename);
        println!(
            "  Words: {}, 
  Lines: {}, 
  Size: {} bytes, 
  Time: {:?}",
            res.stats.word_count,
            res.stats.line_count,
            res.stats.size_bytes,
            res.processing_time,
        );
        println!("  Unique chars: {}", res.stats.char_frequencies.len());
        if !res.errors.is_empty() {
            println!("  Errors:");
            for err in &res.errors {
                println!("    - {}", err.message);
            }
        }
    }

    let final_progress = progress.lock().unwrap();
    println!(
        "\nTotal: {} files, {} processed, {} failed",
        final_progress.total_files,
        final_progress.processed_files,
        final_progress.failed_files
    );
}
