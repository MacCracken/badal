//! Severe weather composite parameters: supercell, tornado, and derecho indices.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Severe weather threat level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ThreatLevel {
    /// No significant threat.
    None,
    /// Marginal — isolated severe weather possible.
    Marginal,
    /// Slight — scattered severe weather possible.
    Slight,
    /// Enhanced — numerous severe weather events likely.
    Enhanced,
    /// Moderate — widespread severe weather expected.
    Moderate,
    /// High — widespread, long-lived, intense severe weather.
    High,
}

impl fmt::Display for ThreatLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Marginal => write!(f, "Marginal"),
            Self::Slight => write!(f, "Slight"),
            Self::Enhanced => write!(f, "Enhanced"),
            Self::Moderate => write!(f, "Moderate"),
            Self::High => write!(f, "High"),
        }
    }
}

/// Supercell Composite Parameter (SCP).
///
/// SCP = (CAPE / 1000) × (shear / 40) × (SRH / 50)
///
/// Normalization follows Thompson et al. (2003) / SPC conventions.
/// SCP > 1 favors supercell development. SCP > 4 favors significant supercells.
///
/// - `cape`: Convective Available Potential Energy (J/kg)
/// - `shear_0_6km`: 0–6 km bulk wind shear magnitude (m/s)
/// - `storm_relative_helicity`: 0–3 km SRH (m²/s²)
#[must_use]
#[inline]
pub fn supercell_composite(cape: f64, shear_0_6km: f64, storm_relative_helicity: f64) -> f64 {
    if cape <= 0.0 || shear_0_6km <= 0.0 || storm_relative_helicity <= 0.0 {
        return 0.0;
    }
    (cape / 1000.0) * (shear_0_6km / 40.0) * (storm_relative_helicity / 50.0)
}

/// Significant Tornado Parameter (STP).
///
/// STP = (CAPE / 1500) × (BWD / 12) × (SRH / 150) × ((2000 - LCL) / 1000)
///
/// Normalization follows Thompson et al. (2012) / SPC conventions.
/// BWD is bulk wind difference (effective-layer shear, m/s), normalized by 12 m/s.
/// Includes LCL height as a low-level moisture proxy. Lower LCL = more favorable.
/// STP > 1 favors significant (EF2+) tornadoes.
///
/// - `cape`: CAPE (J/kg)
/// - `bulk_wind_diff`: effective bulk wind difference (m/s)
/// - `storm_relative_helicity`: 0–3 km SRH (m²/s²)
/// - `lcl_height_m`: Lifting Condensation Level height (m)
#[must_use]
pub fn significant_tornado(
    cape: f64,
    bulk_wind_diff: f64,
    storm_relative_helicity: f64,
    lcl_height_m: f64,
) -> f64 {
    if cape <= 0.0 || bulk_wind_diff <= 0.0 || storm_relative_helicity <= 0.0 {
        return 0.0;
    }
    let lcl_term = ((2000.0 - lcl_height_m) / 1000.0).clamp(0.0, 2.0);
    (cape / 1500.0) * (bulk_wind_diff / 12.0) * (storm_relative_helicity / 150.0) * lcl_term
}

/// Derecho Composite Parameter (DCP), simplified 3-term version.
///
/// DCP = (CAPE / 980) × (0-6km shear / 18) × (0-6km mean wind / 16)
///
/// The full Evans & Doswell (2001) DCP separates DCAPE and MUCAPE into
/// distinct terms; this simplified version uses a single CAPE value.
/// Derechos require moderate instability, moderate shear, and strong mean
/// flow to sustain a long-lived bow echo complex.
/// DCP > 2 favors derecho development.
///
/// - `cape`: CAPE (J/kg)
/// - `shear_0_6km`: 0–6 km bulk wind shear (m/s)
/// - `mean_wind_0_6km`: 0–6 km mean wind speed (m/s)
#[must_use]
#[inline]
pub fn derecho_composite(cape: f64, shear_0_6km: f64, mean_wind_0_6km: f64) -> f64 {
    if cape <= 0.0 || shear_0_6km <= 0.0 || mean_wind_0_6km <= 0.0 {
        return 0.0;
    }
    (cape / 980.0) * (shear_0_6km / 18.0) * (mean_wind_0_6km / 16.0)
}

/// Bulk Richardson Number (BRN).
///
/// BRN = CAPE / (0.5 × shear²)
///
/// Relates buoyancy to shear. BRN 10–45 favors supercells.
/// BRN < 10: too much shear (storms sheared apart).
/// BRN > 45: too little shear (ordinary multicells).
///
/// - `cape`: CAPE (J/kg)
/// - `bulk_shear`: bulk wind shear (m/s)
#[must_use]
#[inline]
pub fn bulk_richardson_number(cape: f64, bulk_shear: f64) -> f64 {
    let denom = 0.5 * bulk_shear * bulk_shear;
    if denom < f64::EPSILON || cape <= 0.0 {
        return 0.0;
    }
    cape / denom
}

/// Energy-Helicity Index (EHI).
///
/// EHI = (CAPE × SRH) / 160000
///
/// Combines instability and rotation. EHI > 1 favors tornadoes,
/// EHI > 2 favors significant tornadoes.
///
/// - `cape`: CAPE (J/kg)
/// - `storm_relative_helicity`: 0–3 km SRH (m²/s²)
#[must_use]
#[inline]
pub fn energy_helicity_index(cape: f64, storm_relative_helicity: f64) -> f64 {
    if cape <= 0.0 || storm_relative_helicity <= 0.0 {
        return 0.0;
    }
    cape * storm_relative_helicity / 160_000.0
}

/// Classify severe weather threat level from composite parameters.
///
/// Uses SCP and STP together for a holistic assessment:
/// - High: STP > 4 or SCP > 8
/// - Moderate: STP > 2 or SCP > 4
/// - Enhanced: STP > 1 or SCP > 2
/// - Slight: SCP > 1
/// - Marginal: SCP > 0.5
/// - None: SCP ≤ 0.5
#[must_use]
pub fn classify_threat(scp: f64, stp: f64) -> ThreatLevel {
    if stp > 4.0 || scp > 8.0 {
        ThreatLevel::High
    } else if stp > 2.0 || scp > 4.0 {
        ThreatLevel::Moderate
    } else if stp > 1.0 || scp > 2.0 {
        ThreatLevel::Enhanced
    } else if scp > 1.0 {
        ThreatLevel::Slight
    } else if scp > 0.5 {
        ThreatLevel::Marginal
    } else {
        ThreatLevel::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- SCP --

    #[test]
    fn scp_typical_supercell() {
        // CAPE 2000, shear 40 m/s, SRH 200 m²/s²
        // SCP = (2000/1000) * (40/40) * (200/50) = 2 * 1 * 4 = 8
        let scp = supercell_composite(2000.0, 40.0, 200.0);
        assert!(
            scp > 2.0,
            "typical supercell environment should give SCP > 2, got {scp}"
        );
    }

    #[test]
    fn scp_weak_environment() {
        // CAPE 500, shear 10, SRH 30 → (0.5)*(0.25)*(0.6) = 0.075
        let scp = supercell_composite(500.0, 10.0, 30.0);
        assert!(scp < 1.0, "weak environment should give SCP < 1, got {scp}");
    }

    #[test]
    fn scp_zero_cape() {
        assert_eq!(supercell_composite(0.0, 25.0, 200.0), 0.0);
    }

    #[test]
    fn scp_zero_shear() {
        assert_eq!(supercell_composite(2000.0, 0.0, 200.0), 0.0);
    }

    #[test]
    fn scp_zero_srh() {
        assert_eq!(supercell_composite(2000.0, 25.0, 0.0), 0.0);
    }

    // -- STP --

    #[test]
    fn stp_tornado_favorable() {
        // CAPE 3000, BWD 20, SRH 300, LCL 800m
        // STP = (3000/1500) * (20/12) * (300/150) * ((2000-800)/1000) = 2 * 1.67 * 2 * 1.2 = 8.0
        let stp = significant_tornado(3000.0, 20.0, 300.0, 800.0);
        assert!(
            stp > 2.0,
            "strong tornado environment should give STP > 2, got {stp}"
        );
    }

    #[test]
    fn stp_high_lcl_unfavorable() {
        // LCL at 2000m → LCL term = 0
        let stp = significant_tornado(3000.0, 20.0, 300.0, 2000.0);
        assert_eq!(stp, 0.0, "LCL at 2000m should zero out STP");
    }

    #[test]
    fn stp_low_lcl_favorable() {
        let high_lcl = significant_tornado(2000.0, 15.0, 200.0, 1500.0);
        let low_lcl = significant_tornado(2000.0, 15.0, 200.0, 500.0);
        assert!(low_lcl > high_lcl, "lower LCL should increase STP");
    }

    #[test]
    fn stp_zero_inputs() {
        assert_eq!(significant_tornado(0.0, 15.0, 200.0, 500.0), 0.0);
        assert_eq!(significant_tornado(2000.0, 0.0, 200.0, 500.0), 0.0);
        assert_eq!(significant_tornado(2000.0, 15.0, 0.0, 500.0), 0.0);
    }

    // -- DCP --

    #[test]
    fn dcp_derecho_favorable() {
        // CAPE 2000, shear 25, mean wind 20
        let dcp = derecho_composite(2000.0, 25.0, 20.0);
        assert!(
            dcp > 2.0,
            "strong derecho environment should give DCP > 2, got {dcp}"
        );
    }

    #[test]
    fn dcp_weak_wind() {
        let dcp = derecho_composite(2000.0, 25.0, 5.0);
        assert!(dcp < 2.0, "weak mean wind should reduce DCP");
    }

    #[test]
    fn dcp_zero_inputs() {
        assert_eq!(derecho_composite(0.0, 25.0, 20.0), 0.0);
        assert_eq!(derecho_composite(2000.0, 0.0, 20.0), 0.0);
        assert_eq!(derecho_composite(2000.0, 25.0, 0.0), 0.0);
    }

    // -- BRN --

    #[test]
    fn brn_supercell_range() {
        // CAPE 2000, shear 20 → BRN = 2000 / (0.5 × 400) = 10
        let brn = bulk_richardson_number(2000.0, 20.0);
        assert!(
            (10.0..=45.0).contains(&brn),
            "BRN should be in supercell range, got {brn}"
        );
    }

    #[test]
    fn brn_multicell() {
        // High CAPE, low shear → BRN > 45
        let brn = bulk_richardson_number(3000.0, 5.0);
        assert!(
            brn > 45.0,
            "low shear should give BRN > 45 (multicell), got {brn}"
        );
    }

    #[test]
    fn brn_zero_shear() {
        assert_eq!(bulk_richardson_number(2000.0, 0.0), 0.0);
    }

    #[test]
    fn brn_zero_cape() {
        assert_eq!(bulk_richardson_number(0.0, 20.0), 0.0);
    }

    // -- EHI --

    #[test]
    fn ehi_tornado_favorable() {
        // CAPE 2000, SRH 200 → EHI = 2.5
        let ehi = energy_helicity_index(2000.0, 200.0);
        assert!((ehi - 2.5).abs() < 0.01, "EHI should be 2.5, got {ehi}");
    }

    #[test]
    fn ehi_zero_inputs() {
        assert_eq!(energy_helicity_index(0.0, 200.0), 0.0);
        assert_eq!(energy_helicity_index(2000.0, 0.0), 0.0);
    }

    // -- threat level --

    #[test]
    fn threat_none() {
        assert_eq!(classify_threat(0.3, 0.0), ThreatLevel::None);
    }

    #[test]
    fn threat_marginal() {
        assert_eq!(classify_threat(0.7, 0.0), ThreatLevel::Marginal);
    }

    #[test]
    fn threat_slight() {
        assert_eq!(classify_threat(1.5, 0.5), ThreatLevel::Slight);
    }

    #[test]
    fn threat_enhanced() {
        assert_eq!(classify_threat(2.5, 1.5), ThreatLevel::Enhanced);
    }

    #[test]
    fn threat_moderate() {
        assert_eq!(classify_threat(5.0, 0.5), ThreatLevel::Moderate);
    }

    #[test]
    fn threat_high() {
        assert_eq!(classify_threat(9.0, 5.0), ThreatLevel::High);
    }

    #[test]
    fn threat_ordering() {
        assert!(ThreatLevel::High > ThreatLevel::Moderate);
        assert!(ThreatLevel::Moderate > ThreatLevel::None);
    }

    // -- Display / serde --

    #[test]
    fn threat_display() {
        assert_eq!(ThreatLevel::Enhanced.to_string(), "Enhanced");
        assert_eq!(ThreatLevel::High.to_string(), "High");
    }

    #[test]
    fn threat_serde_roundtrip() {
        let t = ThreatLevel::Moderate;
        let json = serde_json::to_string(&t).unwrap();
        let t2: ThreatLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(t, t2);
    }
}
