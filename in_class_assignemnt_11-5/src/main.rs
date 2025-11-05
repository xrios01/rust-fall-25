use serde::Deserialize;
use std::error::Error;
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String, 
    status: String,  
}

/// Custom error enum 
#[derive(Debug)]
enum FetchError {
    Http(u16),
    Json(String),
    Network(String),
    Io(std::io::Error),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::Http(code) => write!(f, "HTTP error: {}", code),
            FetchError::Json(e) => write!(f, "JSON parse error: {}", e),
            FetchError::Network(e) => write!(f, "Network error: {}", e),
            FetchError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for FetchError {}
impl From<std::io::Error> for FetchError {
    fn from(e: std::io::Error) -> Self {
        FetchError::Io(e)
    }
}

// Fetch random dog image info
fn fetch_random_dog_image() -> Result<DogImage, FetchError> {
    let url = "https://dog.ceo/api/breeds/image/random";
    let resp = ureq::get(url)
        .call()
        .map_err(|e| FetchError::Network(e.to_string()))?;
    if resp.status() != 200 {
        return Err(FetchError::Http(resp.status()));
    }
    resp.into_json::<DogImage>()
        .map_err(|e| FetchError::Json(e.to_string()))
}

// Download image to "downloads" folder
fn download_image_to_file(
    img_url: &str,
    save_dir: &Path,
    filename_stem: &str,
    index: usize,
) -> Result<String, FetchError> {
    let resp = ureq::get(img_url)
        .call()
        .map_err(|e| FetchError::Network(e.to_string()))?;
    if resp.status() != 200 {
        return Err(FetchError::Http(resp.status()));
    }

    fs::create_dir_all(save_dir)?;
    let filepath = save_dir.join(format!("{}_{}.jpg", filename_stem, index));

    let mut reader = resp.into_reader();
    let mut out = File::create(&filepath)?;
    copy(&mut reader, &mut out)?;

    Ok(filepath.display().to_string())
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Downloader");
    println!("====================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            Ok(dog) => {
                println!("✅ API OK: {}", dog.status);
                let save_dir = Path::new("downloads");
                match download_image_to_file(&dog.message, save_dir, "dog", i) {
                    Ok(path) => println!("💾 Saved: {}", path),
                    Err(e) => eprintln!("❌ Download error: {}", e),
                }
            }
            Err(e) => eprintln!("❌ API error: {}", e),
        }
        println!();
    }

    println!("Done. Check the ./downloads folder for images.");
    Ok(())
}
