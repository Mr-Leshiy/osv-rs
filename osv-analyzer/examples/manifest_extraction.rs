//! Demonstrates manifest parsing and package extraction.
//!
//! Each manifest type uses a dedicated extractor. This example shows how to
//! parse a `Cargo.lock` and a `uv.lock` and print the packages found in each.
//!
//! Run with:
//! ```sh
//! cargo run --example manifest_extraction
//! ```

use osv_analyzer::{Manifest, ManifestError, ManifestType};

fn print_packages(
    label: &str,
    manifest_type: ManifestType,
    data: &[u8],
) -> Result<(), ManifestError> {
    let manifest = Manifest::new(data, manifest_type)?;
    println!("{label} ({} packages):", manifest.len());
    for pkg in manifest.iter() {
        let pkg = pkg.map_err(|e| ManifestError::Extraction(e.to_string()))?;
        println!("  {} {} ({})", pkg.name, pkg.version, pkg.ecosystem);
    }
    Ok(())
}

fn main() -> Result<(), ManifestError> {
    let cargo_lock = include_bytes!("../tests/testdata/manifests/Cargo.lock");
    print_packages("Cargo.lock", ManifestType::Cargo, cargo_lock)?;

    println!();

    let uv_lock = include_bytes!("../tests/testdata/manifests/uv.lock");
    print_packages("uv.lock", ManifestType::Uv, uv_lock)?;

    Ok(())
}
