//! Implements the evaluation algorithm from <https://ossf.github.io/osv-schema/#evaluation>.

use osv_types::{Affected, Event, Range, RangeType};

use crate::{Version, package::ManifestPackage};

/// Implements the evaluation algorithm from <https://ossf.github.io/osv-schema/#evaluation>.
pub fn evaluate(
    p: &ManifestPackage,
    affected: &Affected,
) -> anyhow::Result<bool> {
    let Some(ref pkg) = affected.package else {
        return Ok(false);
    };

    if p.ecosystem.ecosystem() != pkg.ecosystem.ecosystem() {
        return Ok(false);
    }

    if pkg.name != p.name {
        return Ok(false);
    }

    // No version constraints means all versions are affected.
    if affected.versions.is_empty() && affected.ranges.is_empty() {
        return Ok(true);
    }

    // Check the explicit versions list by string equality.
    if affected.versions.iter().any(|v| v == &p.version) {
        return Ok(true);
    }

    // Check semver ranges
    let filtered_ranges = affected
        .ranges
        .iter()
        .filter(|r| r.range_type != RangeType::GIT);

    let ecosystem_str = p.ecosystem.ecosystem().to_string();
    let ecosystem_str = ecosystem_str.as_str();
    let version = Version::new(&p.version, ecosystem_str)?;
    included_in_ranges(&version, filtered_ranges, ecosystem_str)
}

/// Evaluates whether a [`semver::Version`] falls within OSV vulnerability ranges.
/// <https://ossf.github.io/osv-schema/#evaluation>
///
/// Returns `true` if `version` falls within any range accepted by this evaluator.
///
/// # Errors
/// - Cannot parse version String as [`semver::Version`]
pub(crate) fn included_in_ranges<'a, I: Iterator<Item = &'a Range>>(
    version: &Version,
    ranges: I,
    ecosystem_str: &str,
) -> anyhow::Result<bool> {
    for r in ranges {
        anyhow::ensure!(
            r.range_type != RangeType::GIT,
            "GIT range type is not compatible."
        );
        if range_contains(version, &r.events, ecosystem_str)? {
            return Ok(true);
        }
    }
    Ok(false)
}

// Returns `true` if `version` is inside the window described by `events`.
///
/// <https://ossf.github.io/osv-schema/#evaluation>
///
/// Processes events in order, toggling an `affected` flag according to the OSV
/// evaluation algorithm:
/// - [`Event::Introduced`]: marks the start of an affected window (inclusive). The
///   special value `"0"` means the beginning of all versions.
/// - [`Event::Fixed`]: ends the window at this version (exclusive).
/// - [`Event::LastAffected`]: ends the window after this version (inclusive).
/// - [`Event::Limit`]: same as [`Event::Fixed`] — an exclusive upper bound.
///
/// # Errors
/// - Cannot parse `event.version` String as [`semver::Version`]
fn range_contains(
    version: &Version,
    events: &[Event],
    ecosystem_str: &str,
) -> anyhow::Result<bool> {
    let mut is_introduced = false;
    let mut is_fixed = false;
    let mut is_before_limits = true;
    for event in events {
        match event {
            Event::Limit { limit } if version >= &Version::new(limit, ecosystem_str)? => {
                is_before_limits = false;
            },
            Event::Introduced { introduced }
                if introduced == "0" || version >= &Version::new(introduced, ecosystem_str)? =>
            {
                is_introduced = true;
            },
            Event::Fixed { fixed } if version >= &Version::new(fixed, ecosystem_str)? => {
                is_fixed = true;
            },
            Event::LastAffected { last_affected }
                if version > &Version::new(last_affected, ecosystem_str)? =>
            {
                is_fixed = true;
            },
            _ => {},
        }
    }
    Ok(is_before_limits && is_introduced && !is_fixed)
}
