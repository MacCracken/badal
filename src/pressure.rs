use crate::atmosphere::{G, R_AIR, SEA_LEVEL_PRESSURE};

/// Barometric pressure at altitude given sea-level pressure.
#[must_use]
pub fn barometric_pressure(altitude_m: f64, sea_level_pressure: f64, temperature_k: f64) -> f64 {
    if temperature_k <= 0.0 { return 0.0; }
    sea_level_pressure * (-G * altitude_m / (R_AIR * temperature_k)).exp()
}

/// Pressure gradient force per unit mass (m/s²).
///
/// F/m = -(1/ρ) × (dP/dx)
#[must_use]
#[inline]
pub fn pressure_gradient_force(dp_dx: f64, density: f64) -> f64 {
    if density.abs() < f64::EPSILON { return 0.0; }
    -dp_dx / density
}

/// Geostrophic wind speed (balance of Coriolis + pressure gradient).
///
/// V_g = (1/(ρf)) × |dP/dx|
#[must_use]
#[inline]
pub fn geostrophic_wind_speed(dp_dx: f64, density: f64, coriolis: f64) -> f64 {
    let denom = density * coriolis.abs();
    if denom < f64::EPSILON { return 0.0; }
    dp_dx.abs() / denom
}

/// Correct station pressure to sea level.
///
/// P_sl = P_station × exp(g × h / (R × T))
#[must_use]
pub fn sea_level_correction(station_pressure: f64, altitude_m: f64, temperature_k: f64) -> f64 {
    if temperature_k <= 0.0 { return station_pressure; }
    station_pressure * (G * altitude_m / (R_AIR * temperature_k)).exp()
}

/// Altimeter setting (QNH) from field elevation and station pressure.
#[must_use]
pub fn altimeter_setting(station_pressure: f64, field_elevation_m: f64, temperature_k: f64) -> f64 {
    sea_level_correction(station_pressure, field_elevation_m, temperature_k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn barometric_at_sea_level() {
        let p = barometric_pressure(0.0, SEA_LEVEL_PRESSURE, 288.15);
        assert!((p - SEA_LEVEL_PRESSURE).abs() < 1.0);
    }

    #[test]
    fn barometric_decreases_with_altitude() {
        let p0 = barometric_pressure(0.0, SEA_LEVEL_PRESSURE, 288.15);
        let p5k = barometric_pressure(5000.0, SEA_LEVEL_PRESSURE, 255.0);
        assert!(p5k < p0);
    }

    #[test]
    fn pressure_gradient_force_basic() {
        // 1 Pa/m gradient, 1.225 kg/m³ → ~0.82 m/s²
        let f = pressure_gradient_force(1.0, 1.225);
        assert!((f - (-0.816)).abs() < 0.01);
    }

    #[test]
    fn geostrophic_wind_basic() {
        // Typical mid-latitude: dP/dx ~ 0.01 Pa/m, ρ=1.225, f=1e-4
        let v = geostrophic_wind_speed(0.01, 1.225, 1e-4);
        assert!(v > 50.0 && v < 120.0, "geostrophic wind should be reasonable, got {v}");
    }

    #[test]
    fn sea_level_correction_increases_pressure() {
        let p_station = 95_000.0; // station at altitude
        let p_sl = sea_level_correction(p_station, 500.0, 280.0);
        assert!(p_sl > p_station, "sea level pressure should be higher than station at altitude");
    }

    #[test]
    fn zero_altitude_no_correction() {
        let p = sea_level_correction(101_325.0, 0.0, 288.15);
        assert!((p - 101_325.0).abs() < 1.0);
    }
}
