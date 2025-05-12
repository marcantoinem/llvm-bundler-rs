use std::{
    env::set_var,
    error::Error,
    fs::{create_dir, exists},
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

fn decompress_tar_xz_stream(data: Bytes) -> Result<(), Box<dyn std::error::Error>> {
    let cursor = std::io::Cursor::new(data);
    let decoder = XzDecoder::new_parallel(cursor);
    let mut archive = Archive::new(decoder);
    archive.unpack(data_local_dir().ok_or("System not supported".to_string())?)?;
    Ok(())
}

pub fn llvm_path() -> Result<PathBuf, String> {
    data_local_dir()
        .map(|p| p.join(LLVM_CACHE_PREFIX))
        .ok_or("System not supported".to_string())
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
    }
    let libclang_path = llvm_path.join("lib");
    unsafe {
        //The build.rs should not be multithreaded at this point.
        set_var("TABLEGEN_200_PREFIX", &llvm_path);
        set_var("MLIR_SYS_200_PREFIX", &llvm_path);
        set_var("LIBCLANG_PATH", libclang_path);
    }
    Ok(())
}
