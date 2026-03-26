use serde::{Deserialize, Serialize};

/// Dry adiabatic lapse rate (°C/m) = 9.8 °C/km.
pub const DRY_ADIABATIC_LAPSE: f64 = 0.0098;
/// Moist adiabatic lapse rate (°C/m) ≈ 6 °C/km (varies with temperature).
pub const MOIST_ADIABATIC_LAPSE: f64 = 0.006;

/// Atmospheric stability classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StabilityClass {
    /// Environmental lapse < moist adiabatic → very stable (inversions)
    Stable,
    /// Environmental lapse between moist and dry adiabatic → conditionally unstable
    Neutral,
    /// Environmental lapse > dry adiabatic → absolutely unstable
    Unstable,
}

/// Classify atmospheric stability from environmental lapse rate.
///
/// lapse_rate in °C/m (positive = temperature decreasing with altitude).
#[must_use]
pub fn classify_stability(environmental_lapse: f64, is_saturated: bool) -> StabilityClass {
    let reference = if is_saturated { MOIST_ADIABATIC_LAPSE } else { DRY_ADIABATIC_LAPSE };
    if environmental_lapse > reference {
        StabilityClass::Unstable
    } else if (environmental_lapse - reference).abs() < 0.001 {
        StabilityClass::Neutral
    } else {
        StabilityClass::Stable
    }
}

/// Simplified CAPE (Convective Available Potential Energy) from parcel buoyancy.
///
/// CAPE = g × Σ((T_parcel - T_env) / T_env × Δz)
///
/// Returns CAPE in J/kg. Higher = more unstable.
#[must_use]
pub fn cape_simple(parcel_temp_k: f64, env_temp_k: f64, depth_m: f64) -> f64 {
    if env_temp_k <= 0.0 { return 0.0; }
    let buoyancy = (parcel_temp_k - env_temp_k) / env_temp_k;
    if buoyancy <= 0.0 { return 0.0; }
    9.81 * buoyancy * depth_m
}

/// Lifted Index: LI = T_env(500mb) - T_parcel(500mb)
///
/// LI > 0 → stable. LI < 0 → unstable (more negative = more unstable).
#[must_use]
#[inline]
pub fn lifted_index(env_temp_500mb: f64, parcel_temp_500mb: f64) -> f64 {
    env_temp_500mb - parcel_temp_500mb
}

/// K-Index: measure of thunderstorm potential.
///
/// KI = (T_850 - T_500) + Td_850 - (T_700 - Td_700)
///
/// KI > 30 → high thunderstorm risk.
#[must_use]
#[inline]
pub fn k_index(t_850: f64, t_500: f64, td_850: f64, t_700: f64, td_700: f64) -> f64 {
    (t_850 - t_500) + td_850 - (t_700 - td_700)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unstable_atmosphere() {
        // Environmental lapse > dry adiabatic → unstable
        let s = classify_stability(0.012, false);
        assert_eq!(s, StabilityClass::Unstable);
    }

    #[test]
    fn stable_atmosphere() {
        // Environmental lapse < dry adiabatic → stable
        let s = classify_stability(0.005, false);
        assert_eq!(s, StabilityClass::Stable);
    }

    #[test]
    fn conditionally_unstable() {
        // Saturated: lapse between moist and dry → could be unstable if saturated
        let s = classify_stability(0.008, true);
        assert_eq!(s, StabilityClass::Unstable, "saturated with lapse > moist should be unstable");
    }

    #[test]
    fn cape_positive_buoyancy() {
        // Parcel 2K warmer than environment over 1000m
        let cape = cape_simple(302.0, 300.0, 1000.0);
        assert!(cape > 0.0, "positive buoyancy should give positive CAPE");
        assert!((cape - 65.4).abs() < 1.0, "CAPE should be ~65 J/kg, got {cape}");
    }

    #[test]
    fn cape_no_buoyancy() {
        let cape = cape_simple(298.0, 300.0, 1000.0);
        assert_eq!(cape, 0.0, "negative buoyancy → zero CAPE");
    }

    #[test]
    fn lifted_index_stable() {
        let li = lifted_index(-20.0, -25.0); // env warmer than parcel at 500mb
        assert!(li > 0.0, "stable: env warmer → positive LI");
    }

    #[test]
    fn lifted_index_unstable() {
        let li = lifted_index(-25.0, -20.0); // parcel warmer
        assert!(li < 0.0, "unstable: parcel warmer → negative LI");
    }

    #[test]
    fn k_index_high_risk() {
        // Typical high thunderstorm risk values
        let ki = k_index(20.0, -10.0, 15.0, 10.0, 5.0);
        assert!(ki > 30.0, "should indicate high thunderstorm risk, got {ki}");
    }

    #[test]
    fn dry_adiabatic_lapse_value() {
        assert!((DRY_ADIABATIC_LAPSE - 0.0098).abs() < 0.0001);
    }
}
