/// Saturation vapor pressure (Pa) from temperature using Magnus-Tetens formula.
///
/// e_s = 611.2 × exp(17.67 × T / (T + 243.5))
///
/// T in °C.
#[must_use]
#[inline]
pub fn saturation_vapor_pressure(temp_celsius: f64) -> f64 {
    611.2 * (17.67 * temp_celsius / (temp_celsius + 243.5)).exp()
}

/// Mixing ratio: w = 0.622 × e / (P - e)
///
/// e = vapor pressure (Pa), P = total pressure (Pa).
#[must_use]
#[inline]
pub fn mixing_ratio(vapor_pressure: f64, total_pressure: f64) -> f64 {
    let denom = total_pressure - vapor_pressure;
    if denom <= 0.0 { return 0.0; }
    0.622 * vapor_pressure / denom
}

/// Relative humidity: RH = (e / e_s) × 100
#[must_use]
#[inline]
pub fn relative_humidity(actual_vp: f64, saturation_vp: f64) -> f64 {
    if saturation_vp <= 0.0 { return 0.0; }
    (actual_vp / saturation_vp * 100.0).clamp(0.0, 100.0)
}

/// Specific humidity: q = 0.622 × e / (P - 0.378 × e)
#[must_use]
#[inline]
pub fn specific_humidity(vapor_pressure: f64, total_pressure: f64) -> f64 {
    let denom = total_pressure - 0.378 * vapor_pressure;
    if denom <= 0.0 { return 0.0; }
    0.622 * vapor_pressure / denom
}

/// Heat index (°C) — simplified Steadman formula.
///
/// Valid for T > 27°C and RH > 40%.
#[must_use]
pub fn heat_index(temp_celsius: f64, humidity_percent: f64) -> f64 {
    if temp_celsius < 27.0 || humidity_percent < 40.0 {
        return temp_celsius;
    }
    let t = temp_celsius;
    let r = humidity_percent;
    // Rothfusz regression
    let hi = -8.784695 + 1.61139411 * t + 2.338549 * r
        - 0.14611605 * t * r - 0.012308094 * t * t
        - 0.016424828 * r * r + 0.002211732 * t * t * r
        + 0.00072546 * t * r * r - 0.000003582 * t * t * r * r;
    hi
}

/// Wet bulb temperature (°C) — simplified Stull formula.
///
/// Tw ≈ T × arctan(0.151977(RH + 8.313659)^0.5) + arctan(T + RH) - arctan(RH - 1.676331) + 0.00391838 × RH^1.5 × arctan(0.023101 × RH) - 4.686035
#[must_use]
pub fn wet_bulb_temperature(temp_celsius: f64, humidity_percent: f64) -> f64 {
    let t = temp_celsius;
    let rh = humidity_percent;
    t * (0.151977 * (rh + 8.313659).sqrt()).atan()
        + (t + rh).atan()
        - (rh - 1.676331).atan()
        + 0.00391838 * rh.powf(1.5) * (0.023101 * rh).atan()
        - 4.686035
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturation_vp_at_20c() {
        let es = saturation_vapor_pressure(20.0);
        assert!((es - 2338.0).abs() < 50.0, "e_s at 20°C should be ~2338 Pa, got {es}");
    }

    #[test]
    fn saturation_vp_at_0c() {
        let es = saturation_vapor_pressure(0.0);
        assert!((es - 611.2).abs() < 10.0, "e_s at 0°C should be ~611 Pa, got {es}");
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
        assert!(hi > 35.0, "heat index should exceed actual temp in hot/humid, got {hi}");
    }

    #[test]
    fn wet_bulb_below_dry_bulb() {
        let wb = wet_bulb_temperature(30.0, 50.0);
        assert!(wb < 30.0, "wet bulb should be below dry bulb at <100% RH, got {wb}");
    }

    #[test]
    fn specific_humidity_positive() {
        let q = specific_humidity(1000.0, 101_325.0);
        assert!(q > 0.0);
    }
}
