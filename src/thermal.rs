//! Coupling between badal atmospheric conditions and ushma heat transfer.
//!
//! Provides surface energy balance, evapotranspiration, and thermal property
//! conversion functions bridging weather modeling with thermodynamics.
//!
//! Requires the `thermo` feature.

use crate::atmosphere::AtmosphericState;
use crate::error::Result;
use crate::moisture;

/// Latent heat of vaporization for water (J/kg) at ~20°C.
pub const LATENT_HEAT_VAPORIZATION: f64 = 2_450_000.0;

/// Specific heat of air at constant pressure (J/(kg·K)).
pub const CP_AIR: f64 = 1005.0;

/// Psychrometric constant (Pa/K) at standard pressure.
///
/// γ = cₚ × P / (ε × Lᵥ) where ε = 0.622.
pub const PSYCHROMETRIC_GAMMA: f64 = 66.5;

/// Surface energy balance result.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceEnergyBalance {
    /// Net radiation at the surface (W/m²). Positive = warming.
    pub net_radiation: f64,
    /// Ground heat flux (W/m²). Energy conducted into the soil.
    pub ground_heat_flux: f64,
    /// Sensible heat flux (W/m²). Convective heating of the air.
    pub sensible_heat_flux: f64,
    /// Latent heat flux (W/m²). Energy consumed by evaporation.
    pub latent_heat_flux: f64,
    /// Residual (W/m²). Should be ~0 for a closed balance.
    pub residual: f64,
}

/// Compute the surface energy balance.
///
/// R_net = G + H + LE
///
/// - `net_radiation`: net radiation at the surface (W/m²)
/// - `ground_fraction`: fraction of R_net conducted into soil (typically 0.05–0.15)
/// - `bowen_ratio`: ratio of sensible to latent heat flux (H/LE).
///   Dry conditions: 2–5. Moist/vegetated: 0.1–0.5. Over water: ~0.1.
#[must_use]
pub fn surface_energy_balance(
    net_radiation: f64,
    ground_fraction: f64,
    bowen_ratio: f64,
) -> SurfaceEnergyBalance {
    let gf = ground_fraction.clamp(0.0, 1.0);
    let ground_heat_flux = net_radiation * gf;
    let available = net_radiation - ground_heat_flux;

    // R_net - G = H + LE, and H = β × LE
    // So: available = LE × (1 + β), LE = available / (1 + β)
    let denom = 1.0 + bowen_ratio.max(0.0);
    let latent_heat_flux = available / denom;
    let sensible_heat_flux = available - latent_heat_flux;
    let residual = net_radiation - ground_heat_flux - sensible_heat_flux - latent_heat_flux;

    SurfaceEnergyBalance {
        net_radiation,
        ground_heat_flux,
        sensible_heat_flux,
        latent_heat_flux,
        residual,
    }
}

/// Penman-Monteith reference evapotranspiration (mm/day).
///
/// FAO-56 simplified form for a hypothetical grass reference crop:
///
/// ET₀ = (0.408 × Δ × (Rₙ - G) + γ × (900 / (T + 273)) × u₂ × (eₛ - eₐ))
///       / (Δ + γ × (1 + 0.34 × u₂))
///
/// - `state`: atmospheric state (temperature, humidity)
/// - `net_radiation_mj`: net radiation (MJ/m²/day)
/// - `ground_heat_flux_mj`: soil heat flux (MJ/m²/day), often ~0 for daily
/// - `wind_speed_2m`: wind speed at 2m height (m/s)
pub fn penman_monteith_et0(
    state: &AtmosphericState,
    net_radiation_mj: f64,
    ground_heat_flux_mj: f64,
    wind_speed_2m: f64,
) -> f64 {
    let t = state.temperature_celsius();
    let rh = state.humidity_percent();

    // Saturation vapor pressure (kPa)
    let es = moisture::saturation_vapor_pressure(t) / 1000.0;
    // Actual vapor pressure from RH
    let ea = es * rh / 100.0;
    // Slope of saturation vapor pressure curve (kPa/°C)
    let delta = 4098.0 * es / ((t + 237.3) * (t + 237.3));
    // Psychrometric constant (kPa/°C)
    let gamma = 0.0665; // ~66.5 Pa/K = 0.0665 kPa/°C

    let u2 = wind_speed_2m.max(0.0);
    let rn_g = net_radiation_mj - ground_heat_flux_mj;

    let numerator = 0.408 * delta * rn_g + gamma * (900.0 / (t + 273.0)) * u2 * (es - ea);
    let denominator = delta + gamma * (1.0 + 0.34 * u2);

    if denominator <= 0.0 {
        return 0.0;
    }
    (numerator / denominator).max(0.0)
}

/// Evaporation rate from an open water surface (mm/day) using the
/// mass transfer (aerodynamic) method.
///
/// E = f(u) × (eₛ - eₐ)
///
/// where f(u) = a + b × u is a wind function.
///
/// - `state`: atmospheric state (temperature, humidity)
/// - `wind_speed_ms`: wind speed (m/s)
#[must_use]
pub fn open_water_evaporation(state: &AtmosphericState, wind_speed_ms: f64) -> f64 {
    let t = state.temperature_celsius();
    let rh = state.humidity_percent();
    let es = moisture::saturation_vapor_pressure(t) / 1000.0; // kPa
    let ea = es * rh / 100.0;
    let vapor_deficit = (es - ea).max(0.0);

    // Penman wind function: f(u) = 2.6 × (1 + 0.54 × u₂) — mm/day/kPa
    let wind_fn = 2.6 * (1.0 + 0.54 * wind_speed_ms.max(0.0));
    wind_fn * vapor_deficit
}

/// Sensible heat flux (W/m²) from surface-air temperature difference.
///
/// H = ρ × cₚ × Cₕ × u × (Tₛ - Tₐ)
///
/// where Cₕ is the bulk transfer coefficient (~0.003 over land).
///
/// - `surface_temp_k`: surface (skin) temperature (K)
/// - `air_temp_k`: air temperature at reference height (K)
/// - `wind_speed_ms`: wind speed (m/s)
/// - `air_density`: air density (kg/m³)
#[must_use]
#[inline]
pub fn sensible_heat_flux(
    surface_temp_k: f64,
    air_temp_k: f64,
    wind_speed_ms: f64,
    air_density: f64,
) -> f64 {
    let ch = 0.003; // bulk transfer coefficient
    air_density * CP_AIR * ch * wind_speed_ms.max(0.0) * (surface_temp_k - air_temp_k)
}

/// Latent heat flux (W/m²) from surface moisture availability.
///
/// LE = ρ × Lᵥ × Cₑ × u × (qₛ - qₐ)
///
/// Simplified using relative humidity as moisture availability proxy.
///
/// - `state`: atmospheric state
/// - `surface_temp_k`: surface temperature (K)
/// - `wind_speed_ms`: wind speed (m/s)
/// - `moisture_availability`: fraction of surface covered by water [0, 1]
#[must_use]
pub fn latent_heat_flux(
    state: &AtmosphericState,
    surface_temp_k: f64,
    wind_speed_ms: f64,
    moisture_availability: f64,
) -> f64 {
    let ma = moisture_availability.clamp(0.0, 1.0);
    let t_surf_c = surface_temp_k - 273.15;
    let t_air_c = state.temperature_celsius();
    let rh = state.humidity_percent();

    let es_surf = moisture::saturation_vapor_pressure(t_surf_c);
    let ea = moisture::saturation_vapor_pressure(t_air_c) * rh / 100.0;
    let vapor_deficit = (es_surf - ea).max(0.0);

    let ce = 0.003; // bulk transfer coefficient for moisture
    let rho = state.density();
    // Convert vapor pressure deficit to specific humidity deficit: Δq ≈ 0.622 × Δe / P
    let dq = 0.622 * vapor_deficit / state.pressure_pa();

    rho * LATENT_HEAT_VAPORIZATION * ce * wind_speed_ms.max(0.0) * dq * ma
}

/// Conductive heat flux into the ground using ushma (W/m²).
///
/// Q/A = k × (T_surface - T_deep) / depth
///
/// - `surface_temp_k`: surface temperature (K)
/// - `deep_temp_k`: temperature at depth (K)
/// - `depth_m`: depth to deep temperature measurement (m)
/// - `material`: soil thermal material from ushma
pub fn ground_heat_flux(
    surface_temp_k: f64,
    deep_temp_k: f64,
    depth_m: f64,
    material: &ushma::material::ThermalMaterial,
) -> Result<f64> {
    let q = ushma::transfer::conduction(
        material.conductivity,
        1.0, // per unit area
        surface_temp_k,
        deep_temp_k,
        depth_m,
    )
    .map_err(|e| crate::error::BadalError::ComputationError(format!("ground heat flux: {e}")))?;
    Ok(q)
}

/// Radiative cooling rate of a surface (W/m²) using ushma Stefan-Boltzmann.
///
/// Net longwave = ε × σ × (T_surface⁴ - T_sky⁴)
///
/// - `surface_temp_k`: surface temperature (K)
/// - `sky_temp_k`: effective sky temperature (K)
/// - `emissivity`: surface emissivity [0, 1]
pub fn radiative_cooling(surface_temp_k: f64, sky_temp_k: f64, emissivity: f64) -> Result<f64> {
    let q =
        ushma::transfer::radiation(emissivity, 1.0, surface_temp_k, sky_temp_k).map_err(|e| {
            crate::error::BadalError::ComputationError(format!("radiative cooling: {e}"))
        })?;
    Ok(q)
}

/// Thermal diffusivity of air (m²/s) at given conditions.
///
/// α = k / (ρ × cₚ)
///
/// Uses ushma's thermal diffusivity function with atmospheric density.
#[must_use]
#[inline]
pub fn air_thermal_diffusivity(density: f64) -> f64 {
    if density <= 0.0 {
        return 0.0;
    }
    // Thermal conductivity of air ≈ 0.026 W/(m·K) at 20°C
    0.026 / (density * CP_AIR)
}

/// Convective heat transfer coefficient (W/(m²·K)) for atmospheric boundary layer.
///
/// Uses ushma's Nusselt correlations with atmospheric wind speed.
///
/// - `wind_speed_ms`: wind speed (m/s)
/// - `length_scale_m`: characteristic length (m), e.g., building height or fetch
#[must_use]
pub fn atmospheric_htc(wind_speed_ms: f64, length_scale_m: f64) -> f64 {
    if wind_speed_ms <= 0.0 || length_scale_m <= 0.0 {
        return 5.0; // natural convection fallback ~5 W/(m²·K)
    }
    // Forced convection: h ≈ 5.7 + 3.8V for wind over flat surfaces (Jurges formula)
    5.7 + 3.8 * wind_speed_ms
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sea_level() -> AtmosphericState {
        AtmosphericState::sea_level()
    }

    // -- surface energy balance --

    #[test]
    fn energy_balance_closed() {
        let seb = surface_energy_balance(500.0, 0.1, 0.5);
        assert!(
            seb.residual.abs() < 1e-10,
            "energy balance should close, residual = {}",
            seb.residual
        );
    }

    #[test]
    fn energy_balance_partitioning() {
        let seb = surface_energy_balance(500.0, 0.1, 1.0);
        // G = 50, available = 450, β=1 → H = LE = 225
        assert!((seb.ground_heat_flux - 50.0).abs() < 0.1);
        assert!((seb.sensible_heat_flux - 225.0).abs() < 0.1);
        assert!((seb.latent_heat_flux - 225.0).abs() < 0.1);
    }

    #[test]
    fn energy_balance_dry() {
        // High Bowen ratio → mostly sensible
        let seb = surface_energy_balance(500.0, 0.1, 5.0);
        assert!(seb.sensible_heat_flux > seb.latent_heat_flux);
    }

    #[test]
    fn energy_balance_moist() {
        // Low Bowen ratio → mostly latent
        let seb = surface_energy_balance(500.0, 0.1, 0.2);
        assert!(seb.latent_heat_flux > seb.sensible_heat_flux);
    }

    // -- evapotranspiration --

    #[test]
    fn penman_monteith_typical() {
        let state = AtmosphericState::new(298.15, 101_325.0, 50.0, 0.0).unwrap();
        let et0 = penman_monteith_et0(&state, 15.0, 0.0, 2.0);
        // Typical reference ET for summer: 3-8 mm/day
        assert!(
            et0 > 2.0 && et0 < 12.0,
            "ET0 should be ~3-8 mm/day, got {et0}"
        );
    }

    #[test]
    fn penman_monteith_no_radiation() {
        let state = sea_level();
        let et0 = penman_monteith_et0(&state, 0.0, 0.0, 0.0);
        assert!(et0 >= 0.0, "ET0 should be non-negative");
    }

    #[test]
    fn open_water_evaporation_typical() {
        let state = AtmosphericState::new(298.15, 101_325.0, 50.0, 0.0).unwrap();
        let e = open_water_evaporation(&state, 3.0);
        assert!(
            e > 2.0 && e < 15.0,
            "open water evap should be 2-15 mm/day, got {e}"
        );
    }

    #[test]
    fn open_water_evaporation_saturated() {
        // 100% RH → no vapor deficit → no evaporation
        let state = AtmosphericState::new(298.15, 101_325.0, 100.0, 0.0).unwrap();
        let e = open_water_evaporation(&state, 5.0);
        assert!(e.abs() < 0.01, "no evaporation at 100% RH, got {e}");
    }

    // -- sensible heat flux --

    #[test]
    fn sensible_heat_positive_warm_surface() {
        let h = sensible_heat_flux(300.0, 288.0, 5.0, 1.225);
        assert!(
            h > 0.0,
            "warm surface should give upward sensible heat flux"
        );
    }

    #[test]
    fn sensible_heat_negative_cold_surface() {
        let h = sensible_heat_flux(280.0, 288.0, 5.0, 1.225);
        assert!(
            h < 0.0,
            "cold surface should give downward sensible heat flux"
        );
    }

    #[test]
    fn sensible_heat_calm_wind() {
        let h = sensible_heat_flux(300.0, 288.0, 0.0, 1.225);
        assert_eq!(h, 0.0);
    }

    // -- latent heat flux --

    #[test]
    fn latent_heat_positive_moist_surface() {
        let state = AtmosphericState::new(288.15, 101_325.0, 50.0, 0.0).unwrap();
        let le = latent_heat_flux(&state, 293.15, 3.0, 1.0);
        assert!(le > 0.0, "moist warm surface should evaporate, got {le}");
    }

    #[test]
    fn latent_heat_zero_moisture() {
        let state = sea_level();
        let le = latent_heat_flux(&state, 293.15, 3.0, 0.0);
        assert_eq!(le, 0.0, "no moisture availability → no latent flux");
    }

    // -- ground heat flux --

    #[test]
    fn ground_heat_flux_downward() {
        let soil = ushma::material::CONCRETE;
        let q = ground_heat_flux(300.0, 285.0, 0.5, &soil).unwrap();
        assert!(
            q > 0.0,
            "warm surface over cool deep soil should conduct downward"
        );
    }

    // -- radiative cooling --

    #[test]
    fn radiative_cooling_positive() {
        let q = radiative_cooling(300.0, 250.0, 0.95).unwrap();
        assert!(
            q > 0.0,
            "warm surface under cold sky should radiate, got {q}"
        );
    }

    #[test]
    fn radiative_cooling_near_equilibrium() {
        let q = radiative_cooling(288.0, 288.0, 0.95).unwrap();
        assert!(
            q.abs() < 0.1,
            "same temp → near-zero net radiation, got {q}"
        );
    }

    // -- air thermal properties --

    #[test]
    fn air_diffusivity_sea_level() {
        let alpha = air_thermal_diffusivity(1.225);
        // ~0.026 / (1.225 × 1005) ≈ 2.1e-5 m²/s
        assert!(
            alpha > 1e-5 && alpha < 5e-5,
            "air thermal diffusivity should be ~2e-5, got {alpha}"
        );
    }

    #[test]
    fn atmospheric_htc_windy() {
        let h = atmospheric_htc(10.0, 1.0);
        // Jurges: 5.7 + 3.8 × 10 = 43.7
        assert!(
            (h - 43.7).abs() < 0.1,
            "HTC at 10 m/s should be ~43.7, got {h}"
        );
    }

    #[test]
    fn atmospheric_htc_calm() {
        let h = atmospheric_htc(0.0, 1.0);
        assert!(
            (h - 5.0).abs() < 0.1,
            "calm → natural convection fallback ~5"
        );
    }
}
