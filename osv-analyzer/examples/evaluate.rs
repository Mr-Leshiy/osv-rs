//! Demonstrates evaluating a package against OSV vulnerability data.
//!
//! Given a manifest package and an OSV `affected` entry, `evaluate` returns
//! whether the package version falls within any of the affected ranges.
//!
//! Run with:
//! ```sh
//! cargo run --example evaluate
//! ```

use osv_analyzer::{ManifestPackage, evaluate};
use osv_types::{Affected, EcosystemWithSuffix, Event, Package, Range, RangeType};

fn pkg(
    name: &str,
    version: &str,
    ecosystem: &str,
) -> anyhow::Result<ManifestPackage> {
    Ok(ManifestPackage {
        name: name.to_owned(),
        version: version.to_owned(),
        ecosystem: ecosystem.parse()?,
    })
}

fn affected(
    ecosystem: EcosystemWithSuffix,
    name: &str,
    introduced: &str,
    fixed: &str,
) -> Affected {
    Affected {
        package: Some(Package {
            ecosystem,
            name: name.to_owned(),
            purl: None,
        }),
        ranges: vec![Range {
            range_type: RangeType::ECOSYSTEM,
            repo: None,
            events: vec![
                Event::Introduced {
                    introduced: introduced.to_owned(),
                },
                Event::Fixed {
                    fixed: fixed.to_owned(),
                },
            ],
            database_specific: None,
        }],
        versions: vec![],
        severity: vec![],
        ecosystem_specific: None,
        database_specific: None,
    }
}

fn main() -> anyhow::Result<()> {
    let vuln = affected("crates.io".parse()?, "serde", "1.0.0", "1.0.200");

    let cases = [
        ("1.0.100", true),
        ("1.0.200", false),
        ("1.0.210", false),
        ("0.9.0", false),
    ];

    for (version, expected) in cases {
        let p = pkg("serde", version, "crates.io")?;
        let result = evaluate(&p, &vuln)?;
        println!(
            "serde {version} in [1.0.0, 1.0.200): {result} ({})",
            if result == expected {
                "correct"
            } else {
                "UNEXPECTED"
            }
        );
    }

    Ok(())
}
