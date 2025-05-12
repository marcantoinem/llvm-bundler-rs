#![cfg(feature = "bundled")]

use std::error::Error;

use llvm_bundler_rs::bundler::bundle_cache;

fn main() -> Result<(), Box<dyn Error>> {
    bundle_cache()?;
    Ok(())
}
