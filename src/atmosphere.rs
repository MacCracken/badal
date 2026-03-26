use serde::{Deserialize, Serialize};

/// ISA sea level temperature (K).
pub const SEA_LEVEL_TEMP: f64 = 288.15;
/// ISA sea level pressure (Pa).
pub const SEA_LEVEL_PRESSURE: f64 = 101_325.0;
/// ISA sea level density (kg/m³).
pub const SEA_LEVEL_DENSITY: f64 = 1.225;
/// Temperature lapse rate (K/m) in troposphere.
pub const LAPSE_RATE: f64 = 0.0065;
/// Specific gas constant for dry air (J/(kg·K)).
pub const R_AIR: f64 = 287.058;
/// Gravitational acceleration (m/s²).
pub const G: f64 = 9.80665;

/// Complete atmospheric state at a point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtmosphericState {
    pub temperature_k: f64,
    pub pressure_pa: f64,
    pub humidity_percent: f64,
    pub altitude_m: f64,
}

impl AtmosphericState {
    /// Sea level standard conditions.
    #[must_use]
    pub fn sea_level() -> Self {
        Self {
            temperature_k: SEA_LEVEL_TEMP,
            pressure_pa: SEA_LEVEL_PRESSURE,
            humidity_percent: 50.0,
            altitude_m: 0.0,
        }
    }

    /// Standard atmosphere at a given altitude.
    #[must_use]
    pub fn at_altitude(altitude_m: f64) -> Self {
        Self {
            temperature_k: standard_temperature(altitude_m),
            pressure_pa: standard_pressure(altitude_m),
            humidity_percent: 50.0, // default; actual humidity varies
            altitude_m,
        }
    }

    /// Air density at this state from ideal gas law.
    #[must_use]
    #[inline]
    pub fn density(&self) -> f64 {
        air_density(self.pressure_pa, self.temperature_k)
    }

    /// Temperature in Celsius.
    #[must_use]
    #[inline]
    pub fn temperature_celsius(&self) -> f64 {
        self.temperature_k - 273.15
    }
}

/// ISA standard temperature at altitude (troposphere: h ≤ 11000m).
///
/// T = 288.15 - 0.0065 × h
#[must_use]
#[inline]
pub fn standard_temperature(altitude_m: f64) -> f64 {
    if altitude_m <= 11_000.0 {
        SEA_LEVEL_TEMP - LAPSE_RATE * altitude_m
    } else {
        216.65 // tropopause constant
    }
}

/// ISA standard pressure at altitude (troposphere + lower stratosphere).
#[must_use]
pub fn standard_pressure(altitude_m: f64) -> f64 {
    if altitude_m <= 11_000.0 {
        let temp_ratio = standard_temperature(altitude_m) / SEA_LEVEL_TEMP;
        SEA_LEVEL_PRESSURE * temp_ratio.powf(G / (LAPSE_RATE * R_AIR))
    } else {
        let p_tropo = standard_pressure(11_000.0);
        let t_tropo = 216.65;
        p_tropo * (-(G * (altitude_m - 11_000.0)) / (R_AIR * t_tropo)).exp()
    }
}

/// Air density from ideal gas law: ρ = P / (R × T)
#[must_use]
#[inline]
pub fn air_density(pressure_pa: f64, temperature_k: f64) -> f64 {
    if temperature_k <= 0.0 {
        return 0.0;
    }
    pressure_pa / (R_AIR * temperature_k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sea_level_temperature() {
        assert!((standard_temperature(0.0) - 288.15).abs() < 0.01);
    }

    #[test]
    fn tropopause_temperature() {
        assert!((standard_temperature(11_000.0) - 216.65).abs() < 0.01);
    }

    #[test]
    fn above_tropopause_constant() {
        assert!((standard_temperature(15_000.0) - 216.65).abs() < 0.01);
    }

    #[test]
    fn sea_level_pressure() {
        assert!((standard_pressure(0.0) - 101_325.0).abs() < 1.0);
    }

    #[test]
    fn pressure_decreases_with_altitude() {
        assert!(standard_pressure(5000.0) < standard_pressure(0.0));
        assert!(standard_pressure(10_000.0) < standard_pressure(5000.0));
    }

    #[test]
    fn pressure_above_tropopause() {
        let p_11k = standard_pressure(11_000.0);
        let p_15k = standard_pressure(15_000.0);
        let p_20k = standard_pressure(20_000.0);
        assert!(p_15k < p_11k, "pressure should decrease above tropopause");
        assert!(
            p_20k < p_15k,
            "pressure should continue decreasing in stratosphere"
        );
        // At 20km, pressure should be roughly 5500 Pa (standard tables)
        assert!(
            p_20k > 4000.0 && p_20k < 7000.0,
            "pressure at 20km should be ~5500 Pa, got {p_20k}"
        );
    }

    #[test]
    fn sea_level_density() {
        let rho = air_density(SEA_LEVEL_PRESSURE, SEA_LEVEL_TEMP);
        assert!(
            (rho - 1.225).abs() < 0.01,
            "sea level density should be ~1.225, got {rho}"
        );
    }

    #[test]
    fn air_density_zero_temp() {
        assert_eq!(air_density(101_325.0, 0.0), 0.0);
        assert_eq!(air_density(101_325.0, -10.0), 0.0);
    }

    #[test]
    fn ideal_gas_law_holds() {
        let t = standard_temperature(5000.0);
        let p = standard_pressure(5000.0);
        let rho = air_density(p, t);
        let p_check = rho * R_AIR * t;
        assert!((p - p_check).abs() < 1.0, "ideal gas law should hold");
    }

    #[test]
    fn atmospheric_state_sea_level() {
        let s = AtmosphericState::sea_level();
        assert!((s.temperature_k - 288.15).abs() < 0.01);
        assert!((s.density() - 1.225).abs() < 0.01);
    }

    #[test]
    fn atmospheric_state_at_altitude() {
        let s = AtmosphericState::at_altitude(5000.0);
        assert!((s.temperature_k - 255.65).abs() < 0.01);
        assert!(s.pressure_pa < SEA_LEVEL_PRESSURE);
        assert_eq!(s.altitude_m, 5000.0);
        assert_eq!(s.humidity_percent, 50.0);
    }

    #[test]
    fn temperature_celsius_conversion() {
        let s = AtmosphericState::sea_level();
        assert!((s.temperature_celsius() - 15.0).abs() < 0.01);
    }

    #[test]
    fn atmospheric_state_serde_roundtrip() {
        let s = AtmosphericState::sea_level();
        let json = serde_json::to_string(&s).unwrap();
        let s2: AtmosphericState = serde_json::from_str(&json).unwrap();
        assert_eq!(s, s2);
    }
}
