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

fn main() -> Result<(), VersionError> {
    // npm uses SemVer — patch bump
    let v1 = Version::new("1.2.3", "npm")?;
    let v2 = Version::new("1.2.4", "npm")?;
    println!("npm  1.2.3 < 1.2.4 : {}", v1 < v2);

    // PyPI pre-release ordering
    let alpha = Version::new("1.0a1", "PyPI")?;
    let stable = Version::new("1.0", "PyPI")?;
    println!("PyPI 1.0a1 < 1.0   : {}", alpha < stable);

    // Maven epoch-style comparison
    let old = Version::new("1.9", "Maven")?;
    let new = Version::new("1.10", "Maven")?;
    println!("Maven 1.9 < 1.10   : {}", old < new);

    // Versions from different ecosystems are incomparable
    let npm = Version::new("1.0.0", "npm")?;
    let pypi = Version::new("1.0.0", "PyPI")?;
    println!(
        "npm 1.0.0 vs PyPI 1.0.0 incomparable: {}",
        npm.partial_cmp(&pypi).is_none()
    );

    Ok(())
}
