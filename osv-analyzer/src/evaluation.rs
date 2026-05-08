//! Implements the evaluation algorithm from <https://ossf.github.io/osv-schema/#evaluation>.

use osv_types::{Affected, Ecosystem, Event, Range, RangeType};

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

    let version = Version::new(&p.version, p.ecosystem.ecosystem())?;
    included_in_ranges(&version, filtered_ranges, p.ecosystem.ecosystem())
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
    ecosystem: Ecosystem,
) -> anyhow::Result<bool> {
    for r in ranges {
        anyhow::ensure!(
            r.range_type != RangeType::GIT,
            "GIT range type is not compatible."
        );
        if range_contains(version, &r.events, ecosystem)? {
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
    ecosystem: Ecosystem,
) -> anyhow::Result<bool> {
    let mut is_introduced = false;
    let mut is_fixed = false;
    let mut is_before_limits = true;
    for event in events {
        match event {
            Event::Limit { limit } if version >= &Version::new(limit, ecosystem)? => {
                is_before_limits = false;
            },
            Event::Introduced { introduced }
                if introduced == "0" || version >= &Version::new(introduced, ecosystem)? =>
            {
                is_introduced = true;
            },
            Event::Fixed { fixed } if version >= &Version::new(fixed, ecosystem)? => {
                is_fixed = true;
            },
            Event::LastAffected { last_affected }
                if version > &Version::new(last_affected, ecosystem)? =>
            {
                is_fixed = true;
            },
            _ => {},
        }
    }
    Ok(is_before_limits && is_introduced && !is_fixed)
}

#[cfg(test)]
mod tests {
    use osv_types::{Ecosystem, Event};
    use rand::seq::SliceRandom;
    use test_case::test_case;

    use super::range_contains;
    use crate::Version;

    fn introduced(v: &str) -> Event {
        Event::Introduced {
            introduced: v.to_string(),
        }
    }

    fn fixed(v: &str) -> Event {
        Event::Fixed {
            fixed: v.to_string(),
        }
    }

    fn last_affected(v: &str) -> Event {
        Event::LastAffected {
            last_affected: v.to_string(),
        }
    }

    fn limit(v: &str) -> Event {
        Event::Limit {
            limit: v.to_string(),
        }
    }

    #[test_case("1.5.0", Ecosystem::CratesIo, &[] => false; "no events means no affected range")]
    #[test_case("1.5.0", Ecosystem::CratesIo, &[introduced("1.0.0"), fixed("2.0.0")] => true; "version inside introduced-fixed window")]
    #[test_case("1.0.0", Ecosystem::CratesIo, &[introduced("1.0.0"), fixed("2.0.0")] => true; "introduced bound is inclusive")]
    #[test_case("2.0.0", Ecosystem::CratesIo, &[introduced("1.0.0"), fixed("2.0.0")] => false; "fixed bound is exclusive")]
    #[test_case("2.0.1", Ecosystem::CratesIo, &[introduced("1.0.0"), fixed("2.0.0")] => false; "version above fixed bound is unaffected")]
    #[test_case("0.9.0", Ecosystem::CratesIo, &[introduced("1.0.0"), fixed("2.0.0")] => false; "version below introduced bound is unaffected")]
    #[test_case("0.9.0", Ecosystem::CratesIo, &[introduced("0"), fixed("2.0.0")] => true; "introduced zero matches all versions from the beginning")]
    #[test_case("2.0.0", Ecosystem::CratesIo, &[introduced("0"), last_affected("2.0.0")] => true; "last_affected bound is inclusive")]
    #[test_case("2.0.1", Ecosystem::CratesIo, &[introduced("0"), last_affected("2.0.0")] => false; "version above last_affected is unaffected")]
    #[test_case("1.5.9", Ecosystem::CratesIo, &[introduced("0"), limit("2.0.0")] => true; "version inside limit window")]
    #[test_case("2.0.1", Ecosystem::CratesIo, &[introduced("0"), limit("2.0.0")] => false; "limit bound is exclusive")]
    fn range_contains_test(
        ver: &str,
        ecosystem: Ecosystem,
        events: &[Event],
    ) -> bool {
        let ver = Version::new(ver, ecosystem).unwrap();
        let mut events: Vec<Event> = events.to_vec();
        // passing an unordered events list
        events.shuffle(&mut rand::rng());
        range_contains(&ver, &events, ecosystem).unwrap()
    }
}
