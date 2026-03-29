//! Cross-crate bridges — convert primitive values from other AGNOS science crates
//! into badal atmospheric parameters and vice versa.
//!
//! Always available — takes primitive values (f64), no science crate deps.
//!
//! # Architecture
//!
//! ```text
//! pavan     (aerodynamics) ──┐
//! goonj     (acoustics)      ┤
//! garjan    (sound synth)    ┼──> bridge ──> badal atmospheric parameters
//! prakash   (optics)         ┤
//! vanaspati (botany)        ┘
//! ```

use crate::atmosphere;

// ── Pavan bridges (aerodynamics) ───────────────────────────────────────────

/// Convert altitude (m) to ISA air density (kg/m³), temperature (K),
/// and pressure (Pa).
///
/// Returns `(density, temperature, pressure)`.
#[must_use]
pub fn altitude_to_isa(altitude_m: f64) -> (f64, f64, f64) {
    let t = atmosphere::standard_temperature(altitude_m);
    let p = atmosphere::standard_pressure(altitude_m);
    let rho = atmosphere::air_density(p, t);
    (rho, t, p)
}

/// Convert altitude (m) to ISA dynamic viscosity (Pa·s) using
/// Sutherland's law.
///
/// μ = μ_ref × (T/T_ref)^(3/2) × (T_ref + S) / (T + S)
#[must_use]
#[inline]
pub fn altitude_to_viscosity(altitude_m: f64) -> f64 {
    let t = atmosphere::standard_temperature(altitude_m);
    // Sutherland's law for air
    let mu_ref = 1.716e-5; // Pa·s at T_ref
    let t_ref = 273.15; // K
    let s = 110.4; // K (Sutherland constant)
    mu_ref * (t / t_ref).powf(1.5) * (t_ref + s) / (t + s)
}

/// Convert wind profile parameters to a wind speed at target height (m/s).
///
/// Uses logarithmic wind profile: V(z) = V_ref × ln(z/z0) / ln(z_ref/z0).
/// `speed_ref_ms`: reference wind speed.
/// `z_ref_m`: reference measurement height.
/// `z_target_m`: target height.
/// `z0_m`: surface roughness length (grass ≈ 0.03, forest ≈ 1.0).
#[must_use]
#[inline]
pub fn wind_at_height(speed_ref_ms: f64, z_ref_m: f64, z_target_m: f64, z0_m: f64) -> f64 {
    crate::wind::log_wind_profile(speed_ref_ms, z_ref_m, z_target_m, z0_m)
}

// ── Goonj bridges (acoustics) ──────────────────────────────────────────────

/// Convert atmospheric temperature (°C) and humidity (%) to speed of sound (m/s).
///
/// c = 331.3 + 0.606×T + 0.0124×H (simplified humidity correction).
#[must_use]
#[inline]
pub fn atmosphere_to_speed_of_sound(temperature_celsius: f64, humidity_percent: f64) -> f64 {
    331.3 + 0.606 * temperature_celsius + 0.0124 * humidity_percent.clamp(0.0, 100.0)
}

/// Convert atmospheric state to air absorption coefficients at a
/// reference frequency (dB/km).
///
/// Simplified ISO 9613 model: α ≈ f² × (1.84e-11 × T^0.5 / P + ...)
/// Returns absorption in dB/km for the given frequency.
#[must_use]
#[inline]
pub fn atmosphere_to_absorption_db_km(
    temperature_celsius: f64,
    humidity_percent: f64,
    frequency_hz: f64,
) -> f64 {
    let t_k = (temperature_celsius + 273.15).max(1.0);
    let h = humidity_percent.clamp(1.0, 100.0);
    // Simplified model — captures the key dependencies
    let f2 = frequency_hz * frequency_hz;
    let base = 1.84e-11 * t_k.sqrt() * f2;
    let humidity_factor = 0.01 + 0.049 * (h / 100.0);
    (base + humidity_factor * f2 * 1e-8) * 1000.0 // per km
}

// ── Prakash bridges (optics) ───────────────────────────────────────────────

/// Convert atmospheric density at altitude to Rayleigh optical depth factor.
///
/// τ_rayleigh scales with column density, which falls exponentially with altitude.
/// Returns a scale factor (1.0 at sea level).
#[must_use]
#[inline]
pub fn altitude_to_rayleigh_scale(altitude_m: f64) -> f64 {
    // Scale height for Rayleigh scattering ≈ 8.5 km
    (-altitude_m / 8500.0).exp()
}

/// Convert cloud cover fraction (0-1) to diffuse/direct solar radiation split.
///
/// Returns `(direct_fraction, diffuse_fraction)`.
#[must_use]
#[inline]
pub fn cloud_cover_to_radiation_split(cloud_fraction: f64) -> (f64, f64) {
    let cf = cloud_fraction.clamp(0.0, 1.0);
    let direct = 1.0 - 0.9 * cf;
    let diffuse = 0.1 + 0.85 * cf;
    (direct.max(0.0), diffuse.min(1.0))
}

// ── Vanaspati bridges (botany) ─────────────────────────────────────────────

/// Convert atmospheric state to growing condition parameters.
///
/// Returns `(temperature_c, rainfall_rate_mm_hr, solar_radiation_w_m2)`.
/// `precipitation_rate_mm_hr`: current precipitation rate.
/// `solar_elevation_deg`: sun angle above horizon.
#[must_use]
pub fn atmosphere_to_growing_conditions(
    temperature_k: f64,
    precipitation_rate_mm_hr: f64,
    solar_elevation_deg: f64,
) -> (f64, f64, f64) {
    let temp_c = temperature_k - 273.15;
    let rainfall = precipitation_rate_mm_hr.max(0.0);
    // Simplified clear-sky solar radiation
    let solar = if solar_elevation_deg > 0.0 {
        1361.0 * (solar_elevation_deg.to_radians().sin()).max(0.0) * 0.7 // atmospheric attenuation
    } else {
        0.0
    };
    (temp_c, rainfall, solar)
}

/// Convert temperature (°C) and day-of-year to frost risk (0.0–1.0).
///
/// Returns probability of ground frost based on air temperature and season.
#[must_use]
#[inline]
pub fn frost_risk(temperature_celsius: f64, day_of_year: u16) -> f64 {
    // Ground can freeze when air temp < ~3°C
    if temperature_celsius >= 3.0 {
        return 0.0;
    }
    // Higher risk in winter months (northern hemisphere)
    let season_factor = if !(90..=305).contains(&day_of_year) {
        1.0 // winter
    } else if !(120..=275).contains(&day_of_year) {
        0.5 // shoulder season
    } else {
        0.1 // summer (still possible at high altitude)
    };
    let temp_factor = ((3.0 - temperature_celsius) / 8.0).clamp(0.0, 1.0);
    (temp_factor * season_factor).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Pavan ──────────────────────────────────────────────────────────

    #[test]
    fn isa_sea_level() {
        let (rho, t, p) = altitude_to_isa(0.0);
        assert!((t - 288.15).abs() < 0.01);
        assert!((p - 101325.0).abs() < 1.0);
        assert!((rho - 1.225).abs() < 0.01);
    }

    #[test]
    fn isa_10km() {
        let (rho, t, _) = altitude_to_isa(10000.0);
        assert!(t < 230.0); // much colder
        assert!(rho < 0.5); // much thinner
    }

    #[test]
    fn viscosity_sea_level() {
        let mu = altitude_to_viscosity(0.0);
        assert!((mu - 1.789e-5).abs() < 1e-6);
    }

    #[test]
    fn wind_profile_basic() {
        let v = wind_at_height(10.0, 10.0, 10.0, 0.03);
        assert!((v - 10.0).abs() < 0.1); // same height = same speed
    }

    // ── Goonj ──────────────────────────────────────────────────────────

    #[test]
    fn speed_of_sound_standard() {
        let c = atmosphere_to_speed_of_sound(20.0, 50.0);
        assert!((c - 344.0).abs() < 1.0);
    }

    #[test]
    fn absorption_increases_with_frequency() {
        let low = atmosphere_to_absorption_db_km(20.0, 50.0, 500.0);
        let high = atmosphere_to_absorption_db_km(20.0, 50.0, 4000.0);
        assert!(high > low);
    }

    // ── Prakash ────────────────────────────────────────────────────────

    #[test]
    fn rayleigh_sea_level() {
        let scale = altitude_to_rayleigh_scale(0.0);
        assert!((scale - 1.0).abs() < 0.001);
    }

    #[test]
    fn rayleigh_decreases_with_altitude() {
        let high = altitude_to_rayleigh_scale(8500.0);
        assert!((high - 1.0 / std::f64::consts::E).abs() < 0.01);
    }

    #[test]
    fn cloud_cover_clear() {
        let (direct, diffuse) = cloud_cover_to_radiation_split(0.0);
        assert!(direct > 0.9);
        assert!(diffuse < 0.15);
    }

    #[test]
    fn cloud_cover_overcast() {
        let (direct, diffuse) = cloud_cover_to_radiation_split(1.0);
        assert!(direct < 0.15);
        assert!(diffuse > 0.9);
    }

    // ── Vanaspati ──────────────────────────────────────────────────────

    #[test]
    fn growing_conditions_summer() {
        let (temp, rain, solar) = atmosphere_to_growing_conditions(298.15, 0.0, 60.0);
        assert!((temp - 25.0).abs() < 0.01);
        assert_eq!(rain, 0.0);
        assert!(solar > 500.0);
    }

    #[test]
    fn growing_conditions_night() {
        let (_, _, solar) = atmosphere_to_growing_conditions(288.15, 0.0, -10.0);
        assert_eq!(solar, 0.0);
    }

    #[test]
    fn frost_risk_warm() {
        assert_eq!(frost_risk(10.0, 180), 0.0);
    }

    #[test]
    fn frost_risk_cold_winter() {
        let risk = frost_risk(-5.0, 15);
        assert!(risk > 0.5);
    }

    #[test]
    fn frost_risk_cold_summer() {
        let risk = frost_risk(-2.0, 180);
        // Still possible but lower risk in summer
        assert!(risk > 0.0 && risk < 0.5);
    }
}
