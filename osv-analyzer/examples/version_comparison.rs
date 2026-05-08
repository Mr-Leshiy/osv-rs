//! Demonstrates ecosystem-aware version parsing and comparison.
//!
//! Each package ecosystem has its own version ordering rules. This example
//! shows how the same version string can have different semantics depending
//! on the ecosystem, and that versions from different ecosystems cannot be
//! compared.
//!
//! Run with:
//! ```sh
//! cargo run --example version_comparison
//! ```

use osv_analyzer::{Version, VersionError};
use osv_types::Ecosystem;

fn main() -> Result<(), VersionError> {
    // npm uses SemVer — patch bump
    let v1 = Version::new("1.2.3", Ecosystem::Npm)?;
    let v2 = Version::new("1.2.4",  Ecosystem::Npm)?;
    println!("npm  1.2.3 < 1.2.4 : {}", v1 < v2);

    // PyPI pre-release ordering
    let alpha = Version::new("1.0a1",  Ecosystem::PyPI)?;
    let stable = Version::new("1.0", Ecosystem::PyPI)?;
    println!("PyPI 1.0a1 < 1.0   : {}", alpha < stable);

    // Maven epoch-style comparison
    let old = Version::new("1.9", Ecosystem::Maven)?;
    let new = Version::new("1.10", Ecosystem::Maven)?;
    println!("Maven 1.9 < 1.10   : {}", old < new);

    // Versions from different ecosystems are incomparable
    let npm = Version::new("1.0.0", Ecosystem::Npm)?;
    let pypi = Version::new("1.0.0", Ecosystem::PyPI)?;
    println!(
        "npm 1.0.0 vs PyPI 1.0.0 incomparable: {}",
        npm.partial_cmp(&pypi).is_none()
    );

    Ok(())
}
