# File Processor with Custom Thread Pool

This project is a multi-threaded file processing tool written entirely in **pure Rust**, without using async runtimes or external parallelism libraries (no rayon, no tokio). It processes large collections of `.txt` files—such as books downloaded from Project Gutenberg—using a **hand-built thread pool** and computes useful statistics for each file.

## Features

### Custom Thread Pool
- Implemented using only the Rust standard library  
- Worker threads pull jobs from a shared queue  
- Graceful shutdown and job completion guarantees  

### File Analysis
Each `.txt` file is analyzed for:
- Word count  
- Line count  
- Character frequency map  
- File size  
- Processing time  
- Error reporting (invalid or unreadable files)

### Summary Output
A final report lists:
- Total files processed  
- Files that failed  
- Statistics per file  
- Timing information  

### Testing
Includes:
- Unit tests for the thread pool  
- Unit tests for the analyzer  
- Integration tests  
- Performance benchmark test (ignored by default)

## Project Structure

```
final_project_file_processor
 ├── src
 │    ├── main.rs
 │    ├── thread_pool.rs
 │    ├── analysis.rs
 ├── books/
 │    ├── 12345 - ExampleTitle.txt
 ├── Cargo.toml
 └── README.md
```

Place all `.txt` files in the **books/** folder.

## Running the Program

### Build:
```
cargo build
```

### Run with 4 threads:
```
cargo run -- 4 ./books
```

### Recommended (optimized) run:  (were 4 is the number of threads and and ./books is the directory to look in)
```
cargo run --release -- 4 ./books
```

### Example with 8 threads:
```
cargo run --release -- 8 ./books
```

### Example with 8 threads and multiple direcories:
```
cargo run --release -- 8 ./books ./more_books
```

## Running Tests

### All tests:
```
cargo test
```


## Input Files

`.txt` files should be stored in:

```
books/
 ├── 5263 - Title.txt
 ├── 28406 - AnotherTitle.txt
```
  

## Performance Notes

Using `--release` improves speed drastically.  
Thread count should generally match your number of CPU cores.

## Error Handling

The analyzer detects:
- unreadable files  
- missing metadata  
- invalid UTF-8  
- I/O errors  

All errors are attached to each file's result.

## Final Notes

This project demonstrates:
- Concurrency in Rust  
- Designing a custom thread pool  
- Parallelizing file processing  
- Clean, modular system design  
- Testing multi-threaded components  

