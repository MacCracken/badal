/// Earth's angular velocity (rad/s).
pub const EARTH_ANGULAR_VELOCITY: f64 = 7.292e-5;

/// Coriolis parameter: f = 2Ω × sin(φ)
///
/// latitude in radians.
#[must_use]
#[inline]
pub fn coriolis_parameter(latitude_rad: f64) -> f64 {
    2.0 * EARTH_ANGULAR_VELOCITY * latitude_rad.sin()
}

/// Wind chill temperature (°C) — NWS formula.
///
/// Valid for T ≤ 10°C and wind speed ≥ 4.8 km/h.
#[must_use]
pub fn wind_chill(temp_celsius: f64, wind_speed_kmh: f64) -> f64 {
    if temp_celsius > 10.0 || wind_speed_kmh < 4.8 {
        return temp_celsius;
    }
    let v016 = wind_speed_kmh.powf(0.16);
    13.12 + 0.6215 * temp_celsius - 11.37 * v016 + 0.3965 * temp_celsius * v016
}

/// Beaufort scale classification (0–12) from wind speed in m/s.
#[must_use]
pub fn beaufort_scale(wind_speed_ms: f64) -> u8 {
    match wind_speed_ms {
        v if v < 0.5 => 0,   // Calm
        v if v < 1.6 => 1,   // Light air
        v if v < 3.4 => 2,   // Light breeze
        v if v < 5.5 => 3,   // Gentle breeze
        v if v < 8.0 => 4,   // Moderate breeze
        v if v < 10.8 => 5,  // Fresh breeze
        v if v < 13.9 => 6,  // Strong breeze
        v if v < 17.2 => 7,  // High wind
        v if v < 20.8 => 8,  // Gale
        v if v < 24.5 => 9,  // Strong gale
        v if v < 28.5 => 10, // Storm
        v if v < 32.7 => 11, // Violent storm
        _ => 12,             // Hurricane
    }
}

/// Thermal wind — vertical wind shear driven by horizontal temperature gradient.
///
/// Returns approximate wind speed change per 1000m altitude.
#[must_use]
#[inline]
pub fn thermal_wind_shear(temp_gradient_k_per_m: f64, coriolis: f64) -> f64 {
    if coriolis.abs() < f64::EPSILON { return 0.0; }
    // Simplified: ΔV/Δz ≈ (g/fT) × (dT/dx)
    (9.81 / (coriolis.abs() * 280.0)) * temp_gradient_k_per_m.abs() * 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coriolis_at_45_degrees() {
        let f = coriolis_parameter(45.0_f64.to_radians());
        assert!((f - 1.031e-4).abs() < 0.005e-4, "coriolis at 45° should be ~1.03e-4, got {f}");
    }

    #[test]
    fn coriolis_at_equator() {
        let f = coriolis_parameter(0.0);
        assert!(f.abs() < 1e-10, "coriolis at equator should be ~0");
    }

    #[test]
    fn coriolis_at_pole() {
        let f = coriolis_parameter(90.0_f64.to_radians());
        assert!((f - 1.4584e-4).abs() < 0.001e-4, "coriolis at pole should be ~1.46e-4, got {f}");
    }

    #[test]
    fn wind_chill_cold() {
        let wc = wind_chill(-10.0, 30.0);
        assert!(wc < -10.0, "wind chill should be colder, got {wc}");
    }

    #[test]
    fn wind_chill_warm_no_effect() {
        assert!((wind_chill(15.0, 30.0) - 15.0).abs() < 0.01);
    }

    #[test]
    fn wind_chill_low_wind_no_effect() {
        assert!((wind_chill(-10.0, 3.0) - (-10.0)).abs() < 0.01);
    }

    #[test]
    fn beaufort_calm() {
        assert_eq!(beaufort_scale(0.2), 0);
    }

    #[test]
    fn beaufort_hurricane() {
        assert_eq!(beaufort_scale(35.0), 12);
    }

    #[test]
    fn beaufort_moderate() {
        assert_eq!(beaufort_scale(6.0), 4); // Moderate breeze
    }

    #[test]
    fn beaufort_monotonic() {
        for i in 0..12 {
            let low = beaufort_scale(i as f64 * 2.5);
            let high = beaufort_scale((i + 1) as f64 * 2.5);
            assert!(high >= low, "Beaufort should be monotonically increasing");
        }
    }
}
