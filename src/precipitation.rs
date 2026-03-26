//! Precipitation estimation: type classification, rain rate, and accumulation.

use crate::cloud::CloudType;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Precipitation type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PrecipitationType {
    /// No precipitation.
    None,
    /// Light, fine drops (stratiform, low intensity).
    Drizzle,
    /// Liquid precipitation.
    Rain,
    /// Frozen precipitation (ice crystals).
    Snow,
    /// Ice pellets — freezes before reaching surface.
    Sleet,
    /// Liquid drops that freeze on contact with sub-zero surfaces.
    FreezingRain,
    /// Large ice formed in strong convective updrafts.
    Hail,
}

impl fmt::Display for PrecipitationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Drizzle => write!(f, "Drizzle"),
            Self::Rain => write!(f, "Rain"),
            Self::Snow => write!(f, "Snow"),
            Self::Sleet => write!(f, "Sleet"),
            Self::FreezingRain => write!(f, "Freezing Rain"),
            Self::Hail => write!(f, "Hail"),
        }
    }
}

/// Precipitation intensity category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Intensity {
    /// No precipitation (0 mm/hr).
    None,
    /// Trace to light (< 2.5 mm/hr).
    Light,
    /// Moderate (2.5–7.5 mm/hr).
    Moderate,
    /// Heavy (7.5–50 mm/hr).
    Heavy,
    /// Violent (> 50 mm/hr).
    Violent,
}

impl fmt::Display for Intensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Light => write!(f, "Light"),
            Self::Moderate => write!(f, "Moderate"),
            Self::Heavy => write!(f, "Heavy"),
            Self::Violent => write!(f, "Violent"),
        }
    }
}

/// Classify precipitation intensity from rain rate (mm/hr).
///
/// Thresholds follow WMO guidelines:
/// - Light: < 2.5 mm/hr
/// - Moderate: 2.5–7.5 mm/hr
/// - Heavy: 7.5–50 mm/hr
/// - Violent: > 50 mm/hr
#[must_use]
#[inline]
pub fn classify_intensity(rate_mm_hr: f64) -> Intensity {
    match rate_mm_hr {
        r if r <= 0.0 => Intensity::None,
        r if r < 2.5 => Intensity::Light,
        r if r < 7.5 => Intensity::Moderate,
        r if r < 50.0 => Intensity::Heavy,
        _ => Intensity::Violent,
    }
}

/// Estimate rain rate (mm/hr) from cloud type and CAPE (J/kg).
///
/// - **Cumulonimbus** (convective): R ≈ 2 × (CAPE / 100)^0.5, capped at 100 mm/hr.
/// - **Nimbostratus** (stratiform): R ≈ 1 + 0.003 × CAPE, capped at 10 mm/hr.
///   Stratiform precipitation is steadier and less CAPE-dependent.
/// - **Other cloud types**: 0 mm/hr (non-precipitating).
///
/// CAPE values: 0–1000 weak, 1000–2500 moderate, 2500+ strong convection.
#[must_use]
pub fn rain_rate(cloud_type: CloudType, cape_j_kg: f64) -> f64 {
    let cape = cape_j_kg.max(0.0);
    match cloud_type {
        CloudType::Cumulonimbus => {
            // Convective: rate scales with sqrt(CAPE)
            let rate = 2.0 * (cape / 100.0).sqrt();
            rate.min(100.0)
        }
        CloudType::Nimbostratus => {
            // Stratiform: gentle linear scaling
            let rate = 1.0 + 0.003 * cape;
            rate.min(10.0)
        }
        CloudType::Stratus => {
            // Drizzle only: 0.1–0.5 mm/hr regardless of CAPE
            0.3
        }
        _ => 0.0,
    }
}

/// Determine precipitation type from surface conditions.
///
/// Uses wet bulb temperature as the primary discriminator (more reliable than
/// dry bulb for phase determination):
///
/// - **Hail**: CAPE > 2000 J/kg with Cumulonimbus (strong updrafts sustain ice aloft;
///   hail regularly reaches the surface even in warm conditions)
/// - **Snow**: wet bulb ≤ 0°C
/// - **Sleet**: wet bulb 0–1.5°C (partial melting/refreezing)
/// - **Freezing rain**: surface temp ≤ 0°C but wet bulb > 1.5°C (liquid freezes on contact)
/// - **Drizzle**: rate < 0.5 mm/hr
/// - **Rain**: everything else
///
/// Returns [`PrecipitationType::None`] if the cloud type doesn't produce precipitation.
#[must_use]
pub fn precipitation_type(
    cloud_type: CloudType,
    surface_temp_c: f64,
    wet_bulb_c: f64,
    cape_j_kg: f64,
) -> PrecipitationType {
    if !cloud_type.produces_precipitation() {
        return PrecipitationType::None;
    }

    let rate = rain_rate(cloud_type, cape_j_kg);
    if rate <= 0.0 {
        return PrecipitationType::None;
    }

    // Strong convection → hail (hail forms aloft and can reach surface in warm conditions)
    if cape_j_kg > 2000.0 && cloud_type == CloudType::Cumulonimbus {
        return PrecipitationType::Hail;
    }

    // Frozen precipitation
    if wet_bulb_c <= 0.0 {
        return PrecipitationType::Snow;
    }

    // Partial melt zone
    if wet_bulb_c <= 1.5 {
        return PrecipitationType::Sleet;
    }

    // Surface freezing with warm air aloft
    if surface_temp_c <= 0.0 {
        return PrecipitationType::FreezingRain;
    }

    // Liquid precipitation
    if rate < 0.5 {
        return PrecipitationType::Drizzle;
    }

    PrecipitationType::Rain
}

/// Precipitation accumulation (mm) from rate and duration.
///
/// Simple integration: accumulation = rate × duration.
#[must_use]
#[inline]
pub fn accumulation(rate_mm_hr: f64, duration_hours: f64) -> f64 {
    if rate_mm_hr <= 0.0 || duration_hours <= 0.0 {
        return 0.0;
    }
    rate_mm_hr * duration_hours
}

/// Snow accumulation (cm) from liquid-equivalent precipitation (mm).
///
/// Applies a snow-to-liquid ratio (SLR). Typical SLR is 10:1 for average snow,
/// but varies with temperature:
/// - Very cold (< -15°C): ~15:1 (dry, fluffy)
/// - Cold (-15 to -5°C): ~12:1
/// - Near freezing (-5 to 0°C): ~8:1 (wet, heavy)
#[must_use]
#[inline]
pub fn snow_accumulation(liquid_mm: f64, surface_temp_c: f64) -> f64 {
    if liquid_mm <= 0.0 {
        return 0.0;
    }
    let slr = snow_liquid_ratio(surface_temp_c);
    // liquid mm × ratio → snow cm (÷10 to convert mm to cm, ×ratio)
    liquid_mm * slr / 10.0
}

/// Snow-to-liquid ratio based on surface temperature.
///
/// Returns the ratio of snow depth to equivalent liquid water depth.
#[must_use]
#[inline]
pub fn snow_liquid_ratio(surface_temp_c: f64) -> f64 {
    if surface_temp_c < -15.0 {
        15.0
    } else if surface_temp_c < -5.0 {
        12.0
    } else {
        8.0
    }
}

/// Freezing level altitude estimate (m) from surface temperature.
///
/// Assumes standard lapse rate (6.5°C/km). Returns 0.0 if surface is already
/// at or below freezing.
#[must_use]
#[inline]
pub fn freezing_level(surface_temp_c: f64) -> f64 {
    if surface_temp_c <= 0.0 {
        return 0.0;
    }
    // height = temp / lapse_rate, lapse = 6.5°C/km = 0.0065°C/m
    surface_temp_c / 0.0065
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- rain_rate --

    #[test]
    fn rain_rate_cumulonimbus_low_cape() {
        let r = rain_rate(CloudType::Cumulonimbus, 200.0);
        assert!(
            r > 0.0 && r < 5.0,
            "low CAPE Cb should give light rain, got {r}"
        );
    }

    #[test]
    fn rain_rate_cumulonimbus_high_cape() {
        let r = rain_rate(CloudType::Cumulonimbus, 3000.0);
        assert!(r > 10.0, "high CAPE Cb should give heavy rain, got {r}");
    }

    #[test]
    fn rain_rate_cumulonimbus_capped() {
        let r = rain_rate(CloudType::Cumulonimbus, 1_000_000.0);
        assert!(
            (r - 100.0).abs() < f64::EPSILON,
            "Cb rain rate should cap at 100 mm/hr"
        );
    }

    #[test]
    fn rain_rate_nimbostratus() {
        let r = rain_rate(CloudType::Nimbostratus, 500.0);
        assert!(r > 1.0 && r < 10.0, "Ns should give moderate rain, got {r}");
    }

    #[test]
    fn rain_rate_nimbostratus_capped() {
        let r = rain_rate(CloudType::Nimbostratus, 100_000.0);
        assert!(
            (r - 10.0).abs() < f64::EPSILON,
            "Ns rain rate should cap at 10 mm/hr"
        );
    }

    #[test]
    fn rain_rate_non_precipitating() {
        assert_eq!(rain_rate(CloudType::Cirrus, 1000.0), 0.0);
        assert_eq!(rain_rate(CloudType::Cumulus, 1000.0), 0.0);
        assert_eq!(rain_rate(CloudType::Altocumulus, 500.0), 0.0);
    }

    #[test]
    fn rain_rate_stratus_drizzle() {
        let r = rain_rate(CloudType::Stratus, 0.0);
        assert!(
            (r - 0.3).abs() < f64::EPSILON,
            "stratus should give drizzle rate ~0.3 mm/hr"
        );
    }

    #[test]
    fn rain_rate_negative_cape_clamped() {
        let r = rain_rate(CloudType::Cumulonimbus, -500.0);
        assert_eq!(r, 0.0, "negative CAPE should give zero rain");
    }

    #[test]
    fn rain_rate_zero_cape() {
        let r = rain_rate(CloudType::Cumulonimbus, 0.0);
        assert_eq!(r, 0.0);
        let r = rain_rate(CloudType::Nimbostratus, 0.0);
        assert!((r - 1.0).abs() < f64::EPSILON, "Ns base rate is 1 mm/hr");
    }

    // -- precipitation_type --

    #[test]
    fn precip_type_non_precipitating_cloud() {
        assert_eq!(
            precipitation_type(CloudType::Cirrus, 20.0, 15.0, 1000.0),
            PrecipitationType::None
        );
    }

    #[test]
    fn precip_type_rain() {
        assert_eq!(
            precipitation_type(CloudType::Cumulonimbus, 20.0, 15.0, 1000.0),
            PrecipitationType::Rain
        );
    }

    #[test]
    fn precip_type_snow() {
        assert_eq!(
            precipitation_type(CloudType::Nimbostratus, -5.0, -3.0, 200.0),
            PrecipitationType::Snow
        );
    }

    #[test]
    fn precip_type_hail_cold() {
        assert_eq!(
            precipitation_type(CloudType::Cumulonimbus, -2.0, -1.0, 3000.0),
            PrecipitationType::Hail
        );
    }

    #[test]
    fn precip_type_hail_warm() {
        // Hail can fall in warm surface conditions with strong CAPE
        assert_eq!(
            precipitation_type(CloudType::Cumulonimbus, 30.0, 22.0, 3000.0),
            PrecipitationType::Hail
        );
    }

    #[test]
    fn precip_type_sleet() {
        assert_eq!(
            precipitation_type(CloudType::Nimbostratus, 1.0, 1.0, 200.0),
            PrecipitationType::Sleet
        );
    }

    #[test]
    fn precip_type_freezing_rain() {
        // Surface below 0, but wet bulb above 1.5 (warm layer aloft)
        assert_eq!(
            precipitation_type(CloudType::Nimbostratus, -1.0, 2.0, 200.0),
            PrecipitationType::FreezingRain
        );
    }

    #[test]
    fn precip_type_drizzle() {
        // Stratus produces drizzle (rate 0.3 mm/hr < 0.5 threshold)
        let pt = precipitation_type(CloudType::Stratus, 15.0, 10.0, 0.0);
        assert_eq!(pt, PrecipitationType::Drizzle);
    }

    // -- classify_intensity --

    #[test]
    fn intensity_none() {
        assert_eq!(classify_intensity(0.0), Intensity::None);
        assert_eq!(classify_intensity(-1.0), Intensity::None);
    }

    #[test]
    fn intensity_light() {
        assert_eq!(classify_intensity(1.0), Intensity::Light);
        assert_eq!(classify_intensity(2.4), Intensity::Light);
    }

    #[test]
    fn intensity_moderate() {
        assert_eq!(classify_intensity(2.5), Intensity::Moderate);
        assert_eq!(classify_intensity(5.0), Intensity::Moderate);
    }

    #[test]
    fn intensity_heavy() {
        assert_eq!(classify_intensity(10.0), Intensity::Heavy);
        assert_eq!(classify_intensity(49.9), Intensity::Heavy);
    }

    #[test]
    fn intensity_violent() {
        assert_eq!(classify_intensity(50.0), Intensity::Violent);
        assert_eq!(classify_intensity(100.0), Intensity::Violent);
    }

    // -- accumulation --

    #[test]
    fn accumulation_basic() {
        assert!((accumulation(10.0, 2.0) - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn accumulation_zero_rate() {
        assert_eq!(accumulation(0.0, 5.0), 0.0);
    }

    #[test]
    fn accumulation_zero_duration() {
        assert_eq!(accumulation(10.0, 0.0), 0.0);
    }

    #[test]
    fn accumulation_negative_inputs() {
        assert_eq!(accumulation(-5.0, 2.0), 0.0);
        assert_eq!(accumulation(5.0, -2.0), 0.0);
    }

    // -- snow --

    #[test]
    fn snow_accumulation_cold() {
        // 10mm liquid at -20°C → SLR 15 → 15cm snow
        let s = snow_accumulation(10.0, -20.0);
        assert!((s - 15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn snow_accumulation_moderate() {
        // 10mm liquid at -10°C → SLR 12 → 12cm snow
        let s = snow_accumulation(10.0, -10.0);
        assert!((s - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn snow_accumulation_warm() {
        // 10mm liquid at -2°C → SLR 8 → 8cm snow
        let s = snow_accumulation(10.0, -2.0);
        assert!((s - 8.0).abs() < f64::EPSILON);
    }

    #[test]
    fn snow_accumulation_zero_liquid() {
        assert_eq!(snow_accumulation(0.0, -10.0), 0.0);
    }

    #[test]
    fn snow_liquid_ratio_boundaries() {
        assert!((snow_liquid_ratio(-20.0) - 15.0).abs() < f64::EPSILON);
        assert!((snow_liquid_ratio(-10.0) - 12.0).abs() < f64::EPSILON);
        assert!((snow_liquid_ratio(-2.0) - 8.0).abs() < f64::EPSILON);
    }

    // -- freezing_level --

    #[test]
    fn freezing_level_warm_surface() {
        let fl = freezing_level(15.0);
        // 15 / 0.0065 ≈ 2308m
        assert!(
            (fl - 2307.7).abs() < 1.0,
            "freezing level at 15°C should be ~2308m, got {fl}"
        );
    }

    #[test]
    fn freezing_level_cold_surface() {
        assert_eq!(freezing_level(-5.0), 0.0);
        assert_eq!(freezing_level(0.0), 0.0);
    }

    // -- Display --

    #[test]
    fn precip_type_display() {
        assert_eq!(PrecipitationType::FreezingRain.to_string(), "Freezing Rain");
        assert_eq!(PrecipitationType::Snow.to_string(), "Snow");
    }

    #[test]
    fn intensity_display() {
        assert_eq!(Intensity::Heavy.to_string(), "Heavy");
        assert_eq!(Intensity::Violent.to_string(), "Violent");
    }

    // -- serde --

    #[test]
    fn precip_type_serde_roundtrip() {
        let pt = PrecipitationType::Hail;
        let json = serde_json::to_string(&pt).unwrap();
        let pt2: PrecipitationType = serde_json::from_str(&json).unwrap();
        assert_eq!(pt, pt2);
    }

    #[test]
    fn intensity_serde_roundtrip() {
        let i = Intensity::Moderate;
        let json = serde_json::to_string(&i).unwrap();
        let i2: Intensity = serde_json::from_str(&json).unwrap();
        assert_eq!(i, i2);
    }
}
