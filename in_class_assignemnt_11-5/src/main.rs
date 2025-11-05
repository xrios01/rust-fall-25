use serde::Deserialize;
use std::{error::Error, fs, io::Read};

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String, // we print this to avoid the unused-field warning
}

#[derive(Debug)]
enum FetchErr {
    Http(u16),
    Json(String),
    Net(String),
    Io(std::io::Error),
}

impl std::fmt::Display for FetchErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchErr::Http(code) => write!(f, "HTTP error: {code}"),
            FetchErr::Json(msg) => write!(f, "JSON parse error: {msg}"),
            FetchErr::Net(msg)  => write!(f, "Network error: {msg}"),
            FetchErr::Io(e)     => write!(f, "I/O error: {e}"),
        }
    }
}
impl Error for FetchErr {}
impl From<std::io::Error> for FetchErr {
    fn from(e: std::io::Error) -> Self { FetchErr::Io(e) }
}

fn fetch_random_dog_image() -> Result<DogImage, FetchErr> {
    let r = ureq::get("https://dog.ceo/api/breeds/image/random")
        .call()
        .map_err(|e| FetchErr::Net(e.to_string()))?;
    if r.status() != 200 { return Err(FetchErr::Http(r.status())); }
    r.into_json().map_err(|e| FetchErr::Json(e.to_string()))
}

fn download_image(url: &str, i: usize) -> Result<String, FetchErr> {
    let r = ureq::get(url).call().map_err(|e| FetchErr::Net(e.to_string()))?;
    if r.status() != 200 { return Err(FetchErr::Http(r.status())); }

    let mut buf = Vec::new();
    r.into_reader().read_to_end(&mut buf)?;
    fs::create_dir_all("downloads")?;

    let ext = url.rsplit('.').next().unwrap_or("jpg");
    let path = format!("downloads/dog_{i}.{ext}");
    fs::write(&path, buf)?;
    Ok(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Downloader (compact)");
    for i in 1..=5 {
        match fetch_random_dog_image() {
            Ok(d) => {
                println!("api status: {}", d.status); // uses `status`
                match download_image(&d.message, i) {
                    Ok(p) => println!("✅ {i}: saved -> {p}"),
                    Err(e) => eprintln!("❌ {i}: download error: {}", e),
                }
            }
            Err(e) => eprintln!("❌ {i}: api error: {}", e),
        }
    }
    Ok(())
}
