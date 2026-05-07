//! [`osv_analyzer::Version`] tests

#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::unreachable,
    clippy::expect_fun_call,
    clippy::arithmetic_side_effects
)]

use osv_analyzer::Version;
use test_case::test_case;

#[test_case("tests/testdata/versions/semver-versions.txt", "npm")]
#[test_case("tests/testdata/versions/pypi-versions.txt", "PyPI")]
#[test_case("tests/testdata/versions/pypi-versions-generated.txt", "PyPI")]
#[test_case("tests/testdata/versions/debian-versions.txt", "Debian")]
#[test_case("tests/testdata/versions/debian-versions-generated.txt", "Debian")]
#[test_case("tests/testdata/versions/alpine-versions.txt", "Alpine")]
#[test_case("tests/testdata/versions/alpine-versions-generated.txt", "Alpine")]
#[test_case("tests/testdata/versions/maven-versions.txt", "Maven")]
#[test_case("tests/testdata/versions/maven-versions-generated.txt", "Maven")]
#[test_case("tests/testdata/versions/nuget-versions.txt", "NuGet")]
#[test_case("tests/testdata/versions/rubygems-versions.txt", "RubyGems")]
#[test_case("tests/testdata/versions/rubygems-versions-generated.txt", "RubyGems")]
// TODO: enable after https://github.com/google/osv-scalibr/pull/2074 is merged
// #[test_case("tests/testdata/versions/hackage-versions.txt", "Hackage")]
#[test_case("tests/testdata/versions/pub-versions.txt", "Pub")]
#[test_case("tests/testdata/versions/cran-versions.txt", "CRAN")]
#[test_case("tests/testdata/versions/cran-versions-generated.txt", "CRAN")]
#[test_case("tests/testdata/versions/packagist-versions.txt", "Packagist")]
#[test_case(
    "tests/testdata/versions/packagist-versions-generated.txt",
    "Packagist"
)]
#[test_case("tests/testdata/versions/redhat-versions.txt", "Red Hat")]
fn compare_versions(
    path: &str,
    ecosystem: &str,
) {
    let content = std::fs::read_to_string(path).unwrap();

    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
            continue;
        }

        let (lhs, op, rhs) = if let Some((l, r)) = line.split_once(" < ") {
            (l.trim(), "<", r.trim())
        } else if let Some((l, r)) = line.split_once(" > ") {
            (l.trim(), ">", r.trim())
        } else if let Some((l, r)) = line.split_once(" = ") {
            (l.trim(), "=", r.trim())
        } else {
            panic!("{path}:{}: cannot parse line: {line}", i + 1);
        };

        let v1 = Version::new(lhs, ecosystem)
            .expect(&format!("{path}:{}: failed to parse '{lhs}'", i + 1));
        let v2 = Version::new(rhs, ecosystem)
            .expect(&format!("{path}:{}: failed to parse '{rhs}'", i + 1));

        match op {
            "<" => assert!(v1 < v2, "{path}:{}: expected {lhs} < {rhs}", i + 1),
            ">" => assert!(v1 > v2, "{path}:{}: expected {lhs} > {rhs}", i + 1),
            "=" => assert_eq!(v1, v2, "{path}:{}: expected {lhs} = {rhs}", i + 1),
            _ => unreachable!(),
        }
    }
}

#[test]
fn different_ecosystems_are_incomparable() {
    let a = Version::new("1.2.3", "npm").unwrap();
    let b = Version::new("1.2.3", "PyPI").unwrap();
    assert_ne!(a, b);
}
