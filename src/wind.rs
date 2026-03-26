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
    if coriolis.abs() < f64::EPSILON {
        return 0.0;
    }
    // Simplified: ΔV/Δz ≈ (g/fT) × (dT/dx)
    (9.81 / (coriolis.abs() * 280.0)) * temp_gradient_k_per_m.abs() * 1000.0
}

/// Wind direction (degrees, meteorological convention) from u and v components.
///
/// Meteorological convention: direction wind is coming FROM, clockwise from north.
/// 0°/360° = north, 90° = east, 180° = south, 270° = west.
///
/// - `u`: east-west component (m/s, positive = eastward)
/// - `v`: north-south component (m/s, positive = northward)
#[must_use]
#[inline]
pub fn wind_direction(u: f64, v: f64) -> f64 {
    if u.abs() < f64::EPSILON && v.abs() < f64::EPSILON {
        return 0.0; // calm
    }
    // Meteorological convention: direction wind is FROM
    // atan2(-u, -v) gives the angle from north, clockwise
    let dir = (-u).atan2(-v).to_degrees();
    if dir < 0.0 { dir + 360.0 } else { dir }
}

/// Wind speed (m/s) from u and v components.
#[must_use]
#[inline]
pub fn wind_speed(u: f64, v: f64) -> f64 {
    u.hypot(v)
}

/// Logarithmic wind profile — extrapolate wind speed to a different height.
///
/// u(z) = u_ref × ln(z / z₀) / ln(z_ref / z₀)
///
/// - `speed_ref`: measured wind speed at reference height (m/s)
/// - `z_ref`: reference measurement height (m)
/// - `z_target`: target height (m)
/// - `z0`: surface roughness length (m). Typical: 0.03 open terrain, 0.1 suburbs, 1.0 city.
#[must_use]
pub fn log_wind_profile(speed_ref: f64, z_ref: f64, z_target: f64, z0: f64) -> f64 {
    if z0 <= 0.0 || z_ref <= z0 || z_target <= z0 || speed_ref <= 0.0 {
        return 0.0;
    }
    speed_ref * (z_target / z0).ln() / (z_ref / z0).ln()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coriolis_at_45_degrees() {
        let f = coriolis_parameter(45.0_f64.to_radians());
        assert!(
            (f - 1.031e-4).abs() < 0.005e-4,
            "coriolis at 45° should be ~1.03e-4, got {f}"
        );
    }

    #[test]
    fn coriolis_at_equator() {
        let f = coriolis_parameter(0.0);
        assert!(f.abs() < 1e-10, "coriolis at equator should be ~0");
    }

    #[test]
    fn coriolis_at_pole() {
        let f = coriolis_parameter(90.0_f64.to_radians());
        assert!(
            (f - 1.4584e-4).abs() < 0.001e-4,
            "coriolis at pole should be ~1.46e-4, got {f}"
        );
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

    #[test]
    fn thermal_wind_shear_positive() {
        let f = coriolis_parameter(45.0_f64.to_radians());
        let shear = thermal_wind_shear(1e-5, f);
        assert!(
            shear > 0.0,
            "thermal wind shear should be positive, got {shear}"
        );
    }

    #[test]
    fn thermal_wind_shear_zero_coriolis() {
        assert_eq!(thermal_wind_shear(1e-5, 0.0), 0.0);
    }

    #[test]
    fn thermal_wind_shear_zero_gradient() {
        let f = coriolis_parameter(45.0_f64.to_radians());
        assert_eq!(thermal_wind_shear(0.0, f), 0.0);
    }

    // -- wind direction --

    #[test]
    fn wind_direction_from_south() {
        // v positive = blowing northward = wind FROM south = 180°
        let dir = wind_direction(0.0, 1.0);
        assert!(
            (dir - 180.0).abs() < 1.0,
            "southerly wind (v>0) should be ~180°, got {dir}"
        );
    }

    #[test]
    fn wind_direction_from_west() {
        // u positive = blowing eastward = wind FROM west = 270°
        let dir = wind_direction(1.0, 0.0);
        assert!(
            (dir - 270.0).abs() < 1.0,
            "westerly wind (u>0) should be ~270°, got {dir}"
        );
    }

    #[test]
    fn wind_direction_from_north() {
        // v negative = blowing southward = wind FROM north = 360° (or 0°)
        let dir = wind_direction(0.0, -1.0);
        assert!(
            !(1.0..=359.0).contains(&dir),
            "northerly wind should be ~0°/360°, got {dir}"
        );
    }

    #[test]
    fn wind_direction_calm() {
        assert_eq!(wind_direction(0.0, 0.0), 0.0);
    }

    #[test]
    fn wind_speed_basic() {
        assert!((wind_speed(3.0, 4.0) - 5.0).abs() < f64::EPSILON);
    }

    // -- log wind profile --

    #[test]
    fn log_profile_same_height() {
        let v = log_wind_profile(10.0, 10.0, 10.0, 0.03);
        assert!((v - 10.0).abs() < 0.01);
    }

    #[test]
    fn log_profile_higher() {
        let v = log_wind_profile(10.0, 10.0, 50.0, 0.03);
        assert!(v > 10.0, "wind should increase with height, got {v}");
    }

    #[test]
    fn log_profile_lower() {
        let v = log_wind_profile(10.0, 10.0, 2.0, 0.03);
        assert!(v < 10.0, "wind should decrease below ref height, got {v}");
    }
}
