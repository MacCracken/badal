//! Mesoscale wind and temperature phenomena: sea/land breeze, mountain/valley
//! winds, and urban heat island effect.

use std::f64::consts::PI;

/// Estimate sea/land breeze speed (m/s) from land-sea temperature contrast.
///
/// Simplified circulation driven by differential heating:
///
/// V ≈ √(g × h × ΔT / T_mean)
///
/// - `land_temp_k`: land surface temperature (K)
/// - `sea_temp_k`: sea surface temperature (K)
/// - `boundary_layer_depth_m`: depth of the thermal circulation (m), typically 500–1500m
///
/// Returns positive for sea breeze (sea → land, land warmer), negative for land
/// breeze (land → sea, sea warmer). Magnitude is the estimated wind speed.
#[must_use]
pub fn sea_land_breeze(land_temp_k: f64, sea_temp_k: f64, boundary_layer_depth_m: f64) -> f64 {
    let t_mean = (land_temp_k + sea_temp_k) / 2.0;
    if t_mean <= 0.0 || boundary_layer_depth_m <= 0.0 {
        return 0.0;
    }
    let dt = land_temp_k - sea_temp_k;
    let magnitude = (9.81 * boundary_layer_depth_m * dt.abs() / t_mean).sqrt();
    if dt >= 0.0 { magnitude } else { -magnitude }
}

/// Sea breeze front penetration distance (km) from elapsed time.
///
/// Empirical: front advances ~20–40 km inland over a typical daytime cycle (~8 hr).
/// Simplified as d ≈ V_breeze × t × 0.5 (front moves at roughly half the breeze speed).
///
/// - `breeze_speed_ms`: sea breeze wind speed (m/s)
/// - `elapsed_hours`: hours since breeze onset
#[must_use]
#[inline]
pub fn sea_breeze_penetration_km(breeze_speed_ms: f64, elapsed_hours: f64) -> f64 {
    if breeze_speed_ms <= 0.0 || elapsed_hours <= 0.0 {
        return 0.0;
    }
    breeze_speed_ms * elapsed_hours * 3600.0 * 0.5 / 1000.0
}

/// Katabatic (downslope) wind speed (m/s) from slope angle and temperature deficit.
///
/// V ≈ √(2 × g × Δz × ΔT / T_env)
///
/// where Δz = L × sin(slope), the vertical drop along the slope.
///
/// - `slope_angle_rad`: slope angle from horizontal (radians)
/// - `slope_length_m`: length of the slope (m)
/// - `temp_deficit_k`: temperature difference between cold surface air and ambient (K, positive)
/// - `env_temp_k`: ambient environmental temperature (K)
///
/// Katabatic winds form when cold, dense air drains downhill (typically at night).
#[must_use]
pub fn katabatic_wind(
    slope_angle_rad: f64,
    slope_length_m: f64,
    temp_deficit_k: f64,
    env_temp_k: f64,
) -> f64 {
    if env_temp_k <= 0.0 || temp_deficit_k <= 0.0 || slope_length_m <= 0.0 {
        return 0.0;
    }
    let dz = slope_length_m * slope_angle_rad.sin().abs();
    (2.0 * 9.81 * dz * temp_deficit_k / env_temp_k).sqrt()
}

/// Anabatic (upslope) wind speed (m/s) from slope heating.
///
/// Driven by solar heating of slopes — warmed air rises along the incline.
/// Uses the same buoyancy scaling as katabatic but with temperature excess.
///
/// - `slope_angle_rad`: slope angle from horizontal (radians)
/// - `slope_length_m`: length of the slope (m)
/// - `temp_excess_k`: temperature excess of slope-heated air above ambient (K, positive)
/// - `env_temp_k`: ambient environmental temperature (K)
#[must_use]
pub fn anabatic_wind(
    slope_angle_rad: f64,
    slope_length_m: f64,
    temp_excess_k: f64,
    env_temp_k: f64,
) -> f64 {
    // Same physics as katabatic, reversed direction
    katabatic_wind(slope_angle_rad, slope_length_m, temp_excess_k, env_temp_k)
}

/// Mountain-valley wind diurnal cycle phase.
///
/// Returns a value in [-1, 1] representing the dominant wind direction:
/// - Positive: anabatic / upvalley (daytime, solar heating)
/// - Negative: katabatic / downvalley (nighttime, radiative cooling)
/// - 0: transition (dawn/dusk)
///
/// - `solar_hour`: local solar time (0–24)
#[must_use]
#[inline]
pub fn valley_wind_phase(solar_hour: f64) -> f64 {
    // Peak upvalley at ~15:00 solar, peak downvalley at ~03:00
    let angle = 2.0 * PI * (solar_hour - 3.0) / 24.0;
    -angle.cos()
}

/// Urban heat island intensity (°C) from urban-rural characteristics.
///
/// ΔT_UHI ≈ f × (population / 10000)^0.27
///
/// Empirical Oke (1973) relationship. Modified by wind speed (ventilation)
/// and cloud cover (reduces radiative cooling differential).
///
/// - `population`: city population (used as proxy for urban extent)
/// - `wind_speed_ms`: ambient wind speed (m/s) — higher wind disperses heat
/// - `cloud_fraction`: cloud cover [0, 1] — clouds reduce UHI by limiting radiative cooling
#[must_use]
pub fn urban_heat_island(population: f64, wind_speed_ms: f64, cloud_fraction: f64) -> f64 {
    if population <= 0.0 {
        return 0.0;
    }
    // Oke (1973) base relationship
    let base_dt = (population / 10000.0).powf(0.27);

    // Wind ventilation factor: strong wind reduces UHI
    let wind_factor = if wind_speed_ms > 0.5 {
        1.0 / (1.0 + 0.3 * wind_speed_ms)
    } else {
        1.0
    };

    // Cloud reduction: overcast skies reduce radiative cooling differential
    let cf = cloud_fraction.clamp(0.0, 1.0);
    let cloud_factor = 1.0 - 0.6 * cf;

    base_dt * wind_factor * cloud_factor
}

/// Sky view factor (SVF) effect on urban canyon temperature (°C excess).
///
/// Narrow urban canyons with low SVF trap longwave radiation, enhancing the UHI.
///
/// ΔT_canyon ≈ ΔT_UHI × (1 + (1 - SVF))
///
/// - `uhi_intensity`: base urban heat island intensity (°C)
/// - `sky_view_factor`: fraction of sky visible from street level [0, 1]
///   (1 = open field, 0.2–0.4 = dense urban canyon)
#[must_use]
#[inline]
pub fn canyon_temperature_excess(uhi_intensity: f64, sky_view_factor: f64) -> f64 {
    let svf = sky_view_factor.clamp(0.0, 1.0);
    uhi_intensity * (1.0 + (1.0 - svf))
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- sea/land breeze --

    #[test]
    fn sea_breeze_land_warmer() {
        // 5K contrast is more typical for a real sea breeze
        let v = sea_land_breeze(298.0, 293.0, 1000.0);
        assert!(v > 0.0, "land warmer → positive (sea breeze), got {v}");
        assert!(
            v > 2.0 && v < 15.0,
            "sea breeze should be reasonable, got {v}"
        );
    }

    #[test]
    fn land_breeze_sea_warmer() {
        let v = sea_land_breeze(283.0, 293.0, 1000.0);
        assert!(v < 0.0, "sea warmer → negative (land breeze), got {v}");
    }

    #[test]
    fn no_breeze_equal_temps() {
        let v = sea_land_breeze(293.0, 293.0, 1000.0);
        assert_eq!(v, 0.0);
    }

    #[test]
    fn breeze_increases_with_contrast() {
        let weak = sea_land_breeze(295.0, 293.0, 1000.0).abs();
        let strong = sea_land_breeze(303.0, 293.0, 1000.0).abs();
        assert!(strong > weak);
    }

    #[test]
    fn breeze_zero_depth() {
        assert_eq!(sea_land_breeze(303.0, 293.0, 0.0), 0.0);
    }

    // -- penetration --

    #[test]
    fn penetration_distance_typical() {
        // 5 m/s breeze over 6 hours → ~54 km
        let d = sea_breeze_penetration_km(5.0, 6.0);
        assert!(d > 40.0 && d < 60.0, "expected ~54 km, got {d}");
    }

    #[test]
    fn penetration_zero_inputs() {
        assert_eq!(sea_breeze_penetration_km(0.0, 6.0), 0.0);
        assert_eq!(sea_breeze_penetration_km(5.0, 0.0), 0.0);
    }

    // -- katabatic/anabatic --

    #[test]
    fn katabatic_wind_typical() {
        // 10° slope, 2km length, 5K deficit at 280K
        let v = katabatic_wind(10.0_f64.to_radians(), 2000.0, 5.0, 280.0);
        assert!(
            v > 3.0 && v < 15.0,
            "katabatic wind should be 3-15 m/s, got {v}"
        );
    }

    #[test]
    fn katabatic_zero_deficit() {
        assert_eq!(
            katabatic_wind(10.0_f64.to_radians(), 2000.0, 0.0, 280.0),
            0.0
        );
    }

    #[test]
    fn katabatic_steeper_is_faster() {
        let gentle = katabatic_wind(5.0_f64.to_radians(), 2000.0, 5.0, 280.0);
        let steep = katabatic_wind(20.0_f64.to_radians(), 2000.0, 5.0, 280.0);
        assert!(steep > gentle);
    }

    #[test]
    fn anabatic_equals_katabatic_same_params() {
        let k = katabatic_wind(10.0_f64.to_radians(), 2000.0, 3.0, 290.0);
        let a = anabatic_wind(10.0_f64.to_radians(), 2000.0, 3.0, 290.0);
        assert!((k - a).abs() < f64::EPSILON);
    }

    // -- valley wind phase --

    #[test]
    fn valley_wind_phase_afternoon_upvalley() {
        let p = valley_wind_phase(14.0);
        assert!(p > 0.5, "afternoon should be strongly upvalley, got {p}");
    }

    #[test]
    fn valley_wind_phase_night_downvalley() {
        let p = valley_wind_phase(3.0);
        assert!(p < -0.5, "predawn should be strongly downvalley, got {p}");
    }

    #[test]
    fn valley_wind_phase_transition() {
        // Dawn ~09:00 and dusk ~21:00 should be near zero
        let dawn = valley_wind_phase(9.0).abs();
        let dusk = valley_wind_phase(21.0).abs();
        assert!(dawn < 0.3, "dawn should be near transition, got {dawn}");
        assert!(dusk < 0.3, "dusk should be near transition, got {dusk}");
    }

    // -- urban heat island --

    #[test]
    fn uhi_large_city() {
        // Population 1M, calm wind, clear sky
        let dt = urban_heat_island(1_000_000.0, 1.0, 0.0);
        assert!(
            dt > 2.0 && dt < 8.0,
            "large city UHI should be 2-8°C, got {dt}"
        );
    }

    #[test]
    fn uhi_small_town() {
        let dt = urban_heat_island(10_000.0, 1.0, 0.0);
        assert!(dt < 2.0, "small town UHI should be < 2°C, got {dt}");
    }

    #[test]
    fn uhi_zero_population() {
        assert_eq!(urban_heat_island(0.0, 5.0, 0.0), 0.0);
    }

    #[test]
    fn uhi_wind_reduces() {
        let calm = urban_heat_island(500_000.0, 0.5, 0.0);
        let windy = urban_heat_island(500_000.0, 10.0, 0.0);
        assert!(
            windy < calm,
            "wind should reduce UHI: calm={calm}, windy={windy}"
        );
    }

    #[test]
    fn uhi_clouds_reduce() {
        let clear = urban_heat_island(500_000.0, 2.0, 0.0);
        let cloudy = urban_heat_island(500_000.0, 2.0, 0.8);
        assert!(
            cloudy < clear,
            "clouds should reduce UHI: clear={clear}, cloudy={cloudy}"
        );
    }

    // -- canyon --

    #[test]
    fn canyon_open_field() {
        let excess = canyon_temperature_excess(5.0, 1.0);
        assert!(
            (excess - 5.0).abs() < f64::EPSILON,
            "SVF=1 → no canyon amplification"
        );
    }

    #[test]
    fn canyon_narrow_amplifies() {
        let open = canyon_temperature_excess(5.0, 1.0);
        let narrow = canyon_temperature_excess(5.0, 0.3);
        assert!(narrow > open, "narrow canyon should amplify UHI");
    }

    #[test]
    fn canyon_minimum_svf() {
        let excess = canyon_temperature_excess(5.0, 0.0);
        assert!(
            (excess - 10.0).abs() < f64::EPSILON,
            "SVF=0 → 2× amplification"
        );
    }
}
