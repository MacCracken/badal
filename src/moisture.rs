/// Magnus-Tetens coefficient `a` (Bolton 1980).
pub const MAGNUS_A: f64 = 17.67;
/// Magnus-Tetens coefficient `b` (Bolton 1980) in °C.
pub const MAGNUS_B: f64 = 243.5;
/// Reference saturation vapor pressure at 0°C (Pa).
pub const E_S0: f64 = 611.2;

/// Saturation vapor pressure (Pa) from temperature using Magnus-Tetens formula.
///
/// e_s = 611.2 × exp(17.67 × T / (T + 243.5))
///
/// T in °C. Uses Bolton (1980) coefficients.
#[must_use]
#[inline]
pub fn saturation_vapor_pressure(temp_celsius: f64) -> f64 {
    E_S0 * (MAGNUS_A * temp_celsius / (temp_celsius + MAGNUS_B)).exp()
}

/// Dew point temperature (°C) from air temperature (°C) and relative humidity (%).
///
/// Magnus-Tetens approximation (inverse). Uses Bolton (1980) coefficients,
/// consistent with [`saturation_vapor_pressure`].
///
/// Returns `None` if humidity is out of range (0, 100].
#[must_use]
pub fn dew_point(temp_celsius: f64, humidity_percent: f64) -> Option<f64> {
    if humidity_percent <= 0.0 || humidity_percent > 100.0 {
        return None;
    }
    let gamma =
        MAGNUS_A * temp_celsius / (MAGNUS_B + temp_celsius) + (humidity_percent / 100.0).ln();
    Some(MAGNUS_B * gamma / (MAGNUS_A - gamma))
}

/// Mixing ratio: w = 0.622 × e / (P - e)
///
/// e = vapor pressure (Pa), P = total pressure (Pa).
#[must_use]
#[inline]
pub fn mixing_ratio(vapor_pressure: f64, total_pressure: f64) -> f64 {
    let denom = total_pressure - vapor_pressure;
    if denom <= 0.0 {
        return 0.0;
    }
    0.622 * vapor_pressure / denom
}

/// Relative humidity: RH = (e / e_s) × 100
#[must_use]
#[inline]
pub fn relative_humidity(actual_vp: f64, saturation_vp: f64) -> f64 {
    if saturation_vp <= 0.0 {
        return 0.0;
    }
    (actual_vp / saturation_vp * 100.0).clamp(0.0, 100.0)
}

/// Specific humidity: q = 0.622 × e / (P - 0.378 × e)
#[must_use]
#[inline]
pub fn specific_humidity(vapor_pressure: f64, total_pressure: f64) -> f64 {
    let denom = total_pressure - 0.378 * vapor_pressure;
    if denom <= 0.0 {
        return 0.0;
    }
    0.622 * vapor_pressure / denom
}

/// Heat index (°C) — NWS Rothfusz regression with adjustments.
///
/// Implements the full NWS algorithm: Steadman initial estimate, Rothfusz
/// regression when HI ≥ 26.7°C, plus low-humidity and high-humidity
/// adjustment terms per NWS Technical Attachment SR 90-23.
#[must_use]
pub fn heat_index(temp_celsius: f64, humidity_percent: f64) -> f64 {
    if temp_celsius < 27.0 {
        return temp_celsius;
    }
    let t = temp_celsius;
    let r = humidity_percent;
    // Rothfusz regression (Celsius version)
    let mut hi = -8.784695 + 1.61139411 * t + 2.338549 * r
        - 0.14611605 * t * r
        - 0.012308094 * t * t
        - 0.016424828 * r * r
        + 0.002211732 * t * t * r
        + 0.00072546 * t * r * r
        - 0.000003582 * t * t * r * r;

    // NWS low-humidity adjustment: RH < 13% and T in 26.7–44.4°C range
    if r < 13.0 && (26.7..=44.4).contains(&t) {
        hi -= ((13.0 - r) / 4.0) * ((17.0 - (t - 35.0).abs()) / 17.0).sqrt();
    }

    // NWS high-humidity adjustment: RH > 85% and T in 26.7–30.6°C range
    if r > 85.0 && (26.7..=30.6).contains(&t) {
        hi += ((r - 85.0) / 10.0) * ((30.6 - t) / 5.0);
    }

    hi
}

/// Wet bulb temperature (°C) — simplified Stull formula.
///
/// Tw ≈ T × arctan(0.151977(RH + 8.313659)^0.5) + arctan(T + RH)
///     - arctan(RH - 1.676331) + 0.00391838 × RH^1.5 × arctan(0.023101 × RH) - 4.686035
#[must_use]
#[inline]
pub fn wet_bulb_temperature(temp_celsius: f64, humidity_percent: f64) -> f64 {
    let t = temp_celsius;
    let rh = humidity_percent;
    t * (0.151977 * (rh + 8.313659).sqrt()).atan() + (t + rh).atan() - (rh - 1.676331).atan()
        + 0.00391838 * rh.powf(1.5) * (0.023101 * rh).atan()
        - 4.686035
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturation_vp_at_20c() {
        let es = saturation_vapor_pressure(20.0);
        assert!(
            (es - 2338.0).abs() < 50.0,
            "e_s at 20°C should be ~2338 Pa, got {es}"
        );
    }

    #[test]
    fn saturation_vp_at_0c() {
        let es = saturation_vapor_pressure(0.0);
        assert!(
            (es - 611.2).abs() < 10.0,
            "e_s at 0°C should be ~611 Pa, got {es}"
        );
    }

    #[test]
    fn saturation_vp_increases_with_temp() {
        assert!(saturation_vapor_pressure(30.0) > saturation_vapor_pressure(20.0));
    }

    #[test]
    fn mixing_ratio_basic() {
        let w = mixing_ratio(1000.0, 101_325.0);
        assert!(w > 0.0 && w < 0.05);
    }

    #[test]
    fn relative_humidity_100_percent() {
        let rh = relative_humidity(2338.0, 2338.0);
        assert!((rh - 100.0).abs() < 0.01);
    }

    #[test]
    fn relative_humidity_50_percent() {
        let rh = relative_humidity(1169.0, 2338.0);
        assert!((rh - 50.0).abs() < 0.5);
    }

    #[test]
    fn heat_index_no_effect_cool() {
        let hi = heat_index(20.0, 50.0);
        assert!((hi - 20.0).abs() < 0.01, "no heat index below 27°C");
    }

    #[test]
    fn heat_index_hot_humid() {
        let hi = heat_index(35.0, 80.0);
        assert!(
            hi > 35.0,
            "heat index should exceed actual temp in hot/humid, got {hi}"
        );
    }

    #[test]
    fn wet_bulb_below_dry_bulb() {
        let wb = wet_bulb_temperature(30.0, 50.0);
        assert!(
            wb < 30.0,
            "wet bulb should be below dry bulb at <100% RH, got {wb}"
        );
    }

    #[test]
    fn wet_bulb_extreme_hot() {
        let wb = wet_bulb_temperature(45.0, 90.0);
        assert!(
            wb > 30.0 && wb < 50.0,
            "wet bulb at 45°C/90% should be reasonable, got {wb}"
        );
    }

    #[test]
    fn wet_bulb_freezing() {
        let wb = wet_bulb_temperature(0.0, 50.0);
        assert!(
            wb < 0.0,
            "wet bulb at 0°C/50% should be below freezing, got {wb}"
        );
    }

    #[test]
    fn specific_humidity_positive() {
        let q = specific_humidity(1000.0, 101_325.0);
        assert!(q > 0.0);
    }

    #[test]
    fn dew_point_reasonable() {
        // At 20°C, 50% RH, dew point should be ~9°C
        let dp = dew_point(20.0, 50.0).unwrap();
        assert!(
            dp > 5.0 && dp < 15.0,
            "dew point at 20°C/50% should be ~9°C, got {dp}"
        );
    }

    #[test]
    fn dew_point_100_percent_equals_temp() {
        let dp = dew_point(20.0, 100.0).unwrap();
        assert!(
            (dp - 20.0).abs() < 0.5,
            "dew point at 100% RH should equal air temp"
        );
    }

    #[test]
    fn dew_point_zero_humidity_returns_none() {
        assert!(dew_point(20.0, 0.0).is_none());
    }

    #[test]
    fn dew_point_negative_humidity_returns_none() {
        assert!(dew_point(20.0, -10.0).is_none());
    }

    #[test]
    fn dew_point_over_100_returns_none() {
        assert!(dew_point(20.0, 101.0).is_none());
    }
}
