use std::error::Error;

#[cfg(feature = "bundled")]
fn main() -> Result<(), Box<dyn Error>> {
    llvm_bundler_rs::bundler::bundle_cache()?;
    Ok(())
}

#[cfg(not(feature = "bundled"))]
fn main() -> Result<(), Box<dyn Error>> {
    println!("This example needs to be run with the --features bundled");
    Ok(())
}
