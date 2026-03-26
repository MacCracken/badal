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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// ISA standard pressure at altitude (troposphere).
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
    if temperature_k <= 0.0 { return 0.0; }
    pressure_pa / (R_AIR * temperature_k)
}

/// Dew point temperature (°C) from air temperature (°C) and relative humidity (%).
///
/// Magnus-Tetens approximation (inverse).
#[must_use]
pub fn dew_point(temp_celsius: f64, humidity_percent: f64) -> f64 {
    let a = 17.27;
    let b = 237.7;
    let gamma = a * temp_celsius / (b + temp_celsius) + (humidity_percent / 100.0).ln();
    b * gamma / (a - gamma)
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
    fn sea_level_density() {
        let rho = air_density(SEA_LEVEL_PRESSURE, SEA_LEVEL_TEMP);
        assert!((rho - 1.225).abs() < 0.01, "sea level density should be ~1.225, got {rho}");
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
    fn dew_point_reasonable() {
        // At 20°C, 50% RH, dew point should be ~9°C
        let dp = dew_point(20.0, 50.0);
        assert!(dp > 5.0 && dp < 15.0, "dew point at 20°C/50% should be ~9°C, got {dp}");
    }

    #[test]
    fn dew_point_100_percent_equals_temp() {
        let dp = dew_point(20.0, 100.0);
        assert!((dp - 20.0).abs() < 0.5, "dew point at 100% RH should equal air temp");
    }

    #[test]
    fn temperature_celsius_conversion() {
        let s = AtmosphericState::sea_level();
        assert!((s.temperature_celsius() - 15.0).abs() < 0.01);
    }
}
