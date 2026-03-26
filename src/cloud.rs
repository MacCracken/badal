use serde::{Deserialize, Serialize};
use std::fmt;

/// Cloud classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CloudType {
    Cumulus,
    Stratus,
    Cirrus,
    Cumulonimbus,
    Stratocumulus,
    Altostratus,
    Altocumulus,
    Nimbostratus,
    Cirrostratus,
    Cirrocumulus,
}

impl fmt::Display for CloudType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cumulus => write!(f, "Cumulus"),
            Self::Stratus => write!(f, "Stratus"),
            Self::Cirrus => write!(f, "Cirrus"),
            Self::Cumulonimbus => write!(f, "Cumulonimbus"),
            Self::Stratocumulus => write!(f, "Stratocumulus"),
            Self::Altostratus => write!(f, "Altostratus"),
            Self::Altocumulus => write!(f, "Altocumulus"),
            Self::Nimbostratus => write!(f, "Nimbostratus"),
            Self::Cirrostratus => write!(f, "Cirrostratus"),
            Self::Cirrocumulus => write!(f, "Cirrocumulus"),
        }
    }
}

impl CloudType {
    /// Typical base altitude range in meters.
    #[must_use]
    pub fn typical_base_range(&self) -> (f64, f64) {
        match self {
            Self::Cumulus => (500.0, 2000.0),
            Self::Stratus => (0.0, 2000.0),
            Self::Cirrus => (6000.0, 12000.0),
            Self::Cumulonimbus => (500.0, 2000.0),
            Self::Stratocumulus => (500.0, 2000.0),
            Self::Altostratus => (2000.0, 6000.0),
            Self::Altocumulus => (2000.0, 6000.0),
            Self::Nimbostratus => (500.0, 3000.0),
            Self::Cirrostratus => (6000.0, 12000.0),
            Self::Cirrocumulus => (6000.0, 12000.0),
        }
    }

    /// Does this cloud type produce precipitation?
    #[must_use]
    pub fn produces_precipitation(&self) -> bool {
        matches!(self, Self::Cumulonimbus | Self::Nimbostratus)
    }
}

/// Estimate cloud base altitude from surface temperature and dew point.
///
/// Approximation: base ≈ (T - T_d) / 8 × 1000 meters
/// (125m per °C of temperature-dew point spread)
#[must_use]
#[inline]
pub fn cloud_base_altitude(surface_temp_celsius: f64, dew_point_celsius: f64) -> f64 {
    let spread = surface_temp_celsius - dew_point_celsius;
    if spread < 0.0 {
        return 0.0;
    }
    spread / 8.0 * 1000.0
}

/// Lifting condensation level — altitude where air parcel reaches saturation.
///
/// LCL ≈ 125 × (T - T_d) meters (same as cloud base approximation).
#[must_use]
#[inline]
pub fn lifting_condensation_level(temp_celsius: f64, dew_point_celsius: f64) -> f64 {
    cloud_base_altitude(temp_celsius, dew_point_celsius)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cloud_base_typical() {
        // T=25°C, Td=15°C → spread=10 → base ≈ 1250m
        let base = cloud_base_altitude(25.0, 15.0);
        assert!(
            (base - 1250.0).abs() < 10.0,
            "cloud base should be ~1250m, got {base}"
        );
    }

    #[test]
    fn cloud_base_saturated() {
        // T = Td → base at surface
        let base = cloud_base_altitude(20.0, 20.0);
        assert!(base.abs() < 0.01, "saturated air → cloud at surface");
    }

    #[test]
    fn cumulonimbus_precipitates() {
        assert!(CloudType::Cumulonimbus.produces_precipitation());
    }

    #[test]
    fn cirrus_no_precipitation() {
        assert!(!CloudType::Cirrus.produces_precipitation());
    }

    #[test]
    fn cloud_type_base_ranges() {
        let (lo, hi) = CloudType::Cirrus.typical_base_range();
        assert!(lo >= 6000.0 && hi <= 12000.0);
    }

    #[test]
    fn lcl_equals_cloud_base() {
        let cb = cloud_base_altitude(25.0, 15.0);
        let lcl = lifting_condensation_level(25.0, 15.0);
        assert!((cb - lcl).abs() < f64::EPSILON);
    }

    #[test]
    fn negative_spread_zero_base() {
        assert_eq!(cloud_base_altitude(10.0, 15.0), 0.0);
    }

    #[test]
    fn cloud_type_display() {
        assert_eq!(CloudType::Cumulonimbus.to_string(), "Cumulonimbus");
        assert_eq!(CloudType::Cirrus.to_string(), "Cirrus");
    }

    #[test]
    fn cloud_type_serde_roundtrip() {
        let ct = CloudType::Stratocumulus;
        let json = serde_json::to_string(&ct).unwrap();
        let ct2: CloudType = serde_json::from_str(&json).unwrap();
        assert_eq!(ct, ct2);
    }
}
