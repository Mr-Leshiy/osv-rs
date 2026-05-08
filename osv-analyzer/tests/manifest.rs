//! [`osv_analyzer::Manifest`] tests

#![allow(clippy::unwrap_used)]

use osv_analyzer::{Manifest, ManifestPackage, ManifestType};
use test_case::test_case;

fn pkg(
    name: &str,
    version: &str,
    ecosystem: &str,
) -> ManifestPackage {
    ManifestPackage {
        name: name.to_owned(),
        version: version.to_owned(),
        ecosystem: ecosystem.parse().unwrap(),
    }
}

#[test_case(
    "tests/testdata/manifests/019e0749-458c-78b1-96ed-5924437a9854",
    ManifestType::Cargo
    =>
    vec![
        pkg("async-trait", "0.1.89", "crates.io"),
    ]
)]
#[test_case(
    "tests/testdata/manifests/Cargo.lock",
    ManifestType::Cargo
    =>
    vec![
        pkg("async-trait", "0.1.89", "crates.io"),
        pkg("zstd-sys", "2.0.16+zstd.1.5.7", "crates.io")
    ]
)]
#[test_case(
    "tests/testdata/manifests/uv.lock",
    ManifestType::Uv
    =>
    vec![
        pkg("anthropic", "0.84.0", "PyPI"),
        pkg("anyio", "4.12.1", "PyPI")
    ]
)]
fn extract_packages(
    manifest_path: &str,
    m_type: ManifestType,
) -> Vec<ManifestPackage> {
    let data = std::fs::read(manifest_path).unwrap();
    let manifest = Manifest::new(&data, m_type).unwrap();
    manifest.iter().collect::<Result<Vec<_>, _>>().unwrap()
}
