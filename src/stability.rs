use serde::{Deserialize, Serialize};

/// Dry adiabatic lapse rate (°C/m) = 9.8 °C/km.
pub const DRY_ADIABATIC_LAPSE: f64 = 0.0098;
/// Moist adiabatic lapse rate (°C/m) ≈ 6 °C/km (varies with temperature).
pub const MOIST_ADIABATIC_LAPSE: f64 = 0.006;

/// Atmospheric stability classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StabilityClass {
    /// Environmental lapse rate < reference adiabatic → stable (resists vertical motion).
    Stable,
    /// Environmental lapse rate ≈ reference adiabatic → neutral (neither resists nor enhances).
    Neutral,
    /// Environmental lapse rate > reference adiabatic → unstable (enhances vertical motion).
    Unstable,
}

/// Classify atmospheric stability from environmental lapse rate.
///
/// Compares against the dry adiabatic lapse rate (unsaturated) or moist adiabatic
/// lapse rate (saturated). `lapse_rate` in °C/m (positive = temperature decreasing
/// with altitude).
#[must_use]
#[inline]
pub fn classify_stability(environmental_lapse: f64, is_saturated: bool) -> StabilityClass {
    let reference = if is_saturated {
        MOIST_ADIABATIC_LAPSE
    } else {
        DRY_ADIABATIC_LAPSE
    };
    if environmental_lapse > reference {
        StabilityClass::Unstable
    } else if (environmental_lapse - reference).abs() < 0.000_3 {
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
#[inline]
pub fn cape_simple(parcel_temp_k: f64, env_temp_k: f64, depth_m: f64) -> f64 {
    if env_temp_k <= 0.0 {
        return 0.0;
    }
    let buoyancy = (parcel_temp_k - env_temp_k) / env_temp_k;
    if buoyancy <= 0.0 {
        return 0.0;
    }
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

/// Total Totals index: simple instability measure.
///
/// TT = (T_850 - T_500) + (Td_850 - T_500)
///
/// TT > 44 → thunderstorms possible. TT > 52 → severe thunderstorms likely.
#[must_use]
#[inline]
pub fn total_totals(t_850: f64, t_500: f64, td_850: f64) -> f64 {
    (t_850 - t_500) + (td_850 - t_500)
}

/// Convective Inhibition (J/kg) — negative buoyancy a parcel must overcome.
///
/// CIN = g × ((T_env - T_parcel) / T_env) × Δz
///
/// Returns a positive value representing the energy barrier. CIN > 200 J/kg
/// typically prevents storm initiation without a strong forcing mechanism.
///
/// - `parcel_temp_k`: parcel temperature (K)
/// - `env_temp_k`: environmental temperature (K)
/// - `depth_m`: depth of the inhibition layer (m)
#[must_use]
#[inline]
pub fn cin_simple(parcel_temp_k: f64, env_temp_k: f64, depth_m: f64) -> f64 {
    if env_temp_k <= 0.0 || depth_m <= 0.0 {
        return 0.0;
    }
    let buoyancy = (env_temp_k - parcel_temp_k) / env_temp_k;
    if buoyancy <= 0.0 {
        return 0.0; // parcel is already buoyant, no inhibition
    }
    9.81 * buoyancy * depth_m
}

/// Moist adiabatic lapse rate (°C/m) as a function of temperature and pressure.
///
/// Γ_m = Γ_d × (1 + L_v × r_s / (R_d × T)) / (1 + L_v² × r_s / (c_p × R_v × T²))
///
/// where r_s is the saturation mixing ratio. More accurate than the constant
/// [`MOIST_ADIABATIC_LAPSE`], which is valid only near 0°C.
///
/// - `temperature_k`: temperature (K)
/// - `pressure_pa`: pressure (Pa)
#[must_use]
pub fn moist_adiabatic_lapse_rate(temperature_k: f64, pressure_pa: f64) -> f64 {
    if temperature_k <= 0.0 || pressure_pa <= 0.0 {
        return DRY_ADIABATIC_LAPSE;
    }
    let lv = 2_501_000.0; // latent heat of vaporization (J/kg)
    let rv = 461.5; // gas constant for water vapor (J/(kg·K))
    let rd = 287.052_87; // gas constant for dry air (J/(kg·K))
    let cp = 1005.0; // specific heat at constant pressure (J/(kg·K))

    // Saturation mixing ratio
    let t_c = temperature_k - 273.15;
    let es = 611.2 * (17.67 * t_c / (t_c + 243.5)).exp();
    let rs = 0.622 * es / (pressure_pa - es).max(1.0);

    let numerator = 1.0 + lv * rs / (rd * temperature_k);
    let denominator = 1.0 + lv * lv * rs / (cp * rv * temperature_k * temperature_k);
    DRY_ADIABATIC_LAPSE * numerator / denominator
}

/// Brunt-Väisälä frequency squared (s⁻²) — measure of static stability.
///
/// N² = (g / θ) × (dθ/dz)
///
/// N² > 0 → stable (oscillations). N² < 0 → unstable (convection).
/// N² = 0 → neutral.
///
/// - `potential_temp_k`: potential temperature (K)
/// - `d_theta_dz`: vertical gradient of potential temperature (K/m)
#[must_use]
#[inline]
pub fn brunt_vaisala_squared(potential_temp_k: f64, d_theta_dz: f64) -> f64 {
    if potential_temp_k <= 0.0 {
        return 0.0;
    }
    (9.81 / potential_temp_k) * d_theta_dz
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
        assert_eq!(
            s,
            StabilityClass::Unstable,
            "saturated with lapse > moist should be unstable"
        );
    }

    #[test]
    fn cape_positive_buoyancy() {
        // Parcel 2K warmer than environment over 1000m
        let cape = cape_simple(302.0, 300.0, 1000.0);
        assert!(cape > 0.0, "positive buoyancy should give positive CAPE");
        assert!(
            (cape - 65.4).abs() < 1.0,
            "CAPE should be ~65 J/kg, got {cape}"
        );
    }

    #[test]
    fn cape_no_buoyancy() {
        let cape = cape_simple(298.0, 300.0, 1000.0);
        assert_eq!(cape, 0.0, "negative buoyancy → zero CAPE");
    }

    #[test]
    fn cape_zero_env_temp() {
        assert_eq!(cape_simple(300.0, 0.0, 1000.0), 0.0);
        assert_eq!(cape_simple(300.0, -10.0, 1000.0), 0.0);
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
        assert!(
            ki > 30.0,
            "should indicate high thunderstorm risk, got {ki}"
        );
    }

    #[test]
    fn dry_adiabatic_lapse_value() {
        assert!((DRY_ADIABATIC_LAPSE - 0.0098).abs() < 0.0001);
    }

    #[test]
    fn stability_class_serde_roundtrip() {
        let s = StabilityClass::Unstable;
        let json = serde_json::to_string(&s).unwrap();
        let s2: StabilityClass = serde_json::from_str(&json).unwrap();
        assert_eq!(s, s2);
    }

    // -- total totals --

    #[test]
    fn total_totals_severe() {
        // T850=20, T500=-10, Td850=15 → TT = 30 + 25 = 55
        let tt = total_totals(20.0, -10.0, 15.0);
        assert!(
            tt > 52.0,
            "TT > 52 should indicate severe thunderstorms, got {tt}"
        );
    }

    #[test]
    fn total_totals_stable() {
        let tt = total_totals(10.0, -5.0, 5.0);
        assert!(tt < 44.0, "TT < 44 should be stable, got {tt}");
    }

    // -- CIN --

    #[test]
    fn cin_inhibition_present() {
        // Parcel 3K cooler than environment over 500m
        let cin = cin_simple(297.0, 300.0, 500.0);
        assert!(cin > 0.0, "cooler parcel should have CIN, got {cin}");
    }

    #[test]
    fn cin_no_inhibition() {
        // Parcel warmer → no CIN
        let cin = cin_simple(302.0, 300.0, 500.0);
        assert_eq!(cin, 0.0);
    }

    // -- moist adiabatic lapse rate --

    #[test]
    fn moist_lapse_rate_near_zero_c() {
        let gamma = moist_adiabatic_lapse_rate(273.15, 100_000.0);
        // Near 0°C, should be ~5-7°C/km = 0.005-0.007°C/m
        assert!(
            gamma > 0.004 && gamma < 0.008,
            "moist lapse at 0°C should be ~6°C/km, got {:.4}",
            gamma
        );
    }

    #[test]
    fn moist_lapse_rate_warm() {
        let gamma = moist_adiabatic_lapse_rate(303.15, 100_000.0); // 30°C
        // At 30°C, should be ~3.5-4.5°C/km
        assert!(
            gamma < 0.006,
            "moist lapse at 30°C should be < 6°C/km, got {:.4}",
            gamma
        );
    }

    #[test]
    fn moist_lapse_rate_cold() {
        let gamma = moist_adiabatic_lapse_rate(243.15, 100_000.0); // -30°C
        // At -30°C, should approach dry adiabatic (~9°C/km)
        assert!(
            gamma > 0.008,
            "moist lapse at -30°C should approach dry adiabatic, got {:.4}",
            gamma
        );
    }

    #[test]
    fn moist_lapse_always_less_than_dry() {
        for t in [243.15, 263.15, 283.15, 303.15] {
            let gamma = moist_adiabatic_lapse_rate(t, 100_000.0);
            assert!(
                gamma < DRY_ADIABATIC_LAPSE,
                "moist lapse {gamma:.4} should be < dry {DRY_ADIABATIC_LAPSE} at T={t}"
            );
        }
    }

    // -- Brunt-Väisälä --

    #[test]
    fn brunt_vaisala_stable() {
        // Positive dθ/dz → stable → N² > 0
        let n2 = brunt_vaisala_squared(300.0, 0.003);
        assert!(n2 > 0.0, "positive dθ/dz should give N² > 0");
    }

    #[test]
    fn brunt_vaisala_unstable() {
        // Negative dθ/dz → unstable → N² < 0
        let n2 = brunt_vaisala_squared(300.0, -0.003);
        assert!(n2 < 0.0, "negative dθ/dz should give N² < 0");
    }
}
