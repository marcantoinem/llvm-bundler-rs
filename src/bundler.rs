use std::{
    env::set_var,
    error::Error,
    fs::{self, create_dir, exists},
    path::PathBuf,
    time::Duration,
};

use bytes::Bytes;
use dirs::data_local_dir;
use liblzma::bufread::XzDecoder;
use tar::Archive;

const LINUX_ARTIFACT_URL: &str =
    "https://github.com/marcantoinem/llvm-bundler-rs/releases/download/v0.0.1/linux-x64.tar.xz";
const LLVM_CACHE_PREFIX: &str = "llvm";
const FINISH_FILE_MUTEX: &str = "complete";

pub fn llvm_path() -> Result<PathBuf, String> {
    data_local_dir()
        .map(|p| p.join(LLVM_CACHE_PREFIX))
        .ok_or("System not supported".to_string())
}

fn decompress_tar_xz_stream(data: Bytes) -> Result<(), Box<dyn std::error::Error>> {
    let cursor = std::io::Cursor::new(data);
    let decoder = XzDecoder::new_parallel(cursor);
    let mut archive = Archive::new(decoder);
    let local_dir = data_local_dir().ok_or("System not supported".to_string())?;
    archive.unpack(&local_dir)?;
    fs::write(llvm_path()?.join(FINISH_FILE_MUTEX), b"")?;
    Ok(())
}

pub fn bundle_cache() -> Result<(), Box<dyn Error>> {
    let llvm_path = llvm_path()?;
    if !exists(&llvm_path).unwrap_or(false) {
        create_dir(&llvm_path).unwrap();
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60 * 5))
            .build()?;
        let response = client.get(LINUX_ARTIFACT_URL).send()?.bytes()?;
        decompress_tar_xz_stream(response)?;
    } else if !exists(llvm_path.join(FINISH_FILE_MUTEX)).unwrap_or(false) {
        // Is already downloading and extracting
        while !exists(llvm_path.join(FINISH_FILE_MUTEX)).unwrap_or(false) {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    unsafe {
        //The build.rs should not be multithreaded at this point.
        set_var("TABLEGEN_200_PREFIX", &llvm_path);
        set_var("MLIR_SYS_200_PREFIX", &llvm_path);
    }
    Ok(())
}
