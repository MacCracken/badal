//! Radiation budget: solar irradiance, longwave emission, and diurnal temperature cycle.

use std::f64::consts::PI;

/// Solar constant — total solar irradiance at top of atmosphere (W/m²).
///
/// Mean value at 1 AU. Actual varies ±3.4% with Earth-Sun distance.
pub const SOLAR_CONSTANT: f64 = 1361.0;

/// Stefan-Boltzmann constant (W/(m²·K⁴)).
pub const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;

/// Mean Earth albedo (fraction reflected, dimensionless).
pub const EARTH_ALBEDO: f64 = 0.30;

/// Atmospheric transmissivity under clear sky (dimensionless).
///
/// Fraction of solar radiation reaching the surface through a clear atmosphere.
/// Typical range 0.6–0.8; varies with aerosol loading and water vapor.
pub const CLEAR_SKY_TRANSMISSIVITY: f64 = 0.75;

/// Effective atmospheric emissivity for longwave (dimensionless).
///
/// Used in the atmospheric window approximation. Varies with humidity and CO₂;
/// 0.75 is a reasonable mid-latitude average.
pub const ATMOSPHERIC_EMISSIVITY: f64 = 0.75;

/// Solar zenith angle from latitude, declination, and hour angle (radians).
///
/// cos(θ_z) = sin(φ)sin(δ) + cos(φ)cos(δ)cos(h)
///
/// - `latitude_rad`: observer latitude (radians, positive north)
/// - `declination_rad`: solar declination (radians)
/// - `hour_angle_rad`: hour angle (radians, 0 = solar noon, negative = morning)
///
/// Returns the zenith angle in radians [0, π]. Values > π/2 mean the sun is
/// below the horizon.
#[must_use]
#[inline]
pub fn solar_zenith_angle(latitude_rad: f64, declination_rad: f64, hour_angle_rad: f64) -> f64 {
    let cos_z = latitude_rad.sin() * declination_rad.sin()
        + latitude_rad.cos() * declination_rad.cos() * hour_angle_rad.cos();
    cos_z.clamp(-1.0, 1.0).acos()
}

/// Solar declination angle (radians) from day of year.
///
/// δ ≈ -23.44° × cos(360/365 × (day + 10))
///
/// Approximation accurate to ±1°.
#[must_use]
#[inline]
pub fn solar_declination(day_of_year: u16) -> f64 {
    let angle = 2.0 * PI / 365.0 * (day_of_year as f64 + 10.0);
    -23.44_f64.to_radians() * angle.cos()
}

/// Hour angle (radians) from solar time.
///
/// h = (hour - 12) × 15° converted to radians.
/// Negative in morning, positive in afternoon, 0 at solar noon.
#[must_use]
#[inline]
pub fn hour_angle(solar_hour: f64) -> f64 {
    (solar_hour - 12.0) * 15.0_f64.to_radians()
}

/// Day length in hours from latitude and solar declination.
///
/// Computed from the sunrise hour angle: cos(h₀) = -tan(φ) × tan(δ).
/// Returns 0 for polar night, 24 for midnight sun.
///
/// - `latitude_rad`: observer latitude (radians)
/// - `declination_rad`: solar declination (radians)
#[must_use]
pub fn day_length(latitude_rad: f64, declination_rad: f64) -> f64 {
    let cos_h0 = -(latitude_rad.tan() * declination_rad.tan());
    if cos_h0 <= -1.0 {
        return 24.0; // midnight sun
    }
    if cos_h0 >= 1.0 {
        return 0.0; // polar night
    }
    let h0 = cos_h0.acos(); // sunrise hour angle in radians
    2.0 * h0.to_degrees() / 15.0 // convert to hours
}

/// Sunrise and sunset hour (solar time) from latitude and declination.
///
/// Returns `(sunrise, sunset)` in solar hours. For polar night returns `(12.0, 12.0)`,
/// for midnight sun returns `(0.0, 24.0)`.
#[must_use]
pub fn sunrise_sunset(latitude_rad: f64, declination_rad: f64) -> (f64, f64) {
    let cos_h0 = -(latitude_rad.tan() * declination_rad.tan());
    if cos_h0 <= -1.0 {
        return (0.0, 24.0); // midnight sun
    }
    if cos_h0 >= 1.0 {
        return (12.0, 12.0); // polar night
    }
    let h0_hours = cos_h0.acos().to_degrees() / 15.0;
    (12.0 - h0_hours, 12.0 + h0_hours)
}

/// Clear-sky solar irradiance at the surface (W/m²).
///
/// I = S × τ × cos(θ_z)
///
/// Returns 0 when the sun is below the horizon (θ_z > π/2).
///
/// - `zenith_rad`: solar zenith angle (radians)
/// - `transmissivity`: atmospheric transmissivity [0, 1]
#[must_use]
#[inline]
pub fn clear_sky_irradiance(zenith_rad: f64, transmissivity: f64) -> f64 {
    let cos_z = zenith_rad.cos();
    if cos_z <= 0.0 {
        return 0.0;
    }
    SOLAR_CONSTANT * transmissivity * cos_z
}

/// Cloud-attenuated solar irradiance at the surface (W/m²).
///
/// I_cloudy = I_clear × (1 - 0.75 × cloud_fraction^3.4)
///
/// Kasten & Czeplak (1980) empirical cloud attenuation.
///
/// - `clear_irradiance`: clear-sky irradiance (W/m²)
/// - `cloud_fraction`: cloud cover fraction [0, 1]
#[must_use]
#[inline]
pub fn cloud_attenuated_irradiance(clear_irradiance: f64, cloud_fraction: f64) -> f64 {
    let cf = cloud_fraction.clamp(0.0, 1.0);
    clear_irradiance * (1.0 - 0.75 * cf.powf(3.4))
}

/// Longwave (infrared) radiation emitted by a surface (W/m²).
///
/// Stefan-Boltzmann law: F = ε × σ × T⁴
///
/// - `emissivity`: surface emissivity [0, 1]
/// - `surface_temp_k`: surface temperature (K)
#[must_use]
#[inline]
pub fn longwave_emission(emissivity: f64, surface_temp_k: f64) -> f64 {
    if surface_temp_k <= 0.0 {
        return 0.0;
    }
    emissivity * STEFAN_BOLTZMANN * surface_temp_k.powi(4)
}

/// Downwelling longwave radiation from the atmosphere (W/m²).
///
/// F_atm = ε_atm × σ × T_atm⁴
///
/// - `atmospheric_emissivity`: effective emissivity of atmosphere [0, 1]
/// - `air_temp_k`: near-surface air temperature (K)
#[must_use]
#[inline]
pub fn atmospheric_longwave(atmospheric_emissivity: f64, air_temp_k: f64) -> f64 {
    if air_temp_k <= 0.0 {
        return 0.0;
    }
    atmospheric_emissivity * STEFAN_BOLTZMANN * air_temp_k.powi(4)
}

/// Net radiation at the surface (W/m²).
///
/// R_net = (1 - α) × S_in + L_down - L_up
///
/// - `shortwave_in`: incoming solar irradiance at surface (W/m²)
/// - `albedo`: surface albedo [0, 1]
/// - `longwave_down`: downwelling atmospheric longwave (W/m²)
/// - `longwave_up`: upwelling surface longwave emission (W/m²)
///
/// Positive = net energy gain (warming), negative = net loss (cooling).
#[must_use]
#[inline]
pub fn net_radiation(shortwave_in: f64, albedo: f64, longwave_down: f64, longwave_up: f64) -> f64 {
    (1.0 - albedo) * shortwave_in + longwave_down - longwave_up
}

/// Estimated diurnal temperature range (°C) from net radiation.
///
/// Simplified energy balance: ΔT ≈ R_net_peak × Δt / (ρ × c_p × h_mix)
///
/// Uses typical mixed-layer depth and heat capacity. Returns the approximate
/// temperature swing from overnight minimum to afternoon maximum.
///
/// - `net_radiation_peak`: peak net radiation (W/m²), typically midday
/// - `cloud_fraction`: cloud cover [0, 1] — clouds damp the diurnal range
#[must_use]
pub fn diurnal_temperature_range(net_radiation_peak: f64, cloud_fraction: f64) -> f64 {
    if net_radiation_peak <= 0.0 {
        return 0.0;
    }
    // Mixed layer: ~1000m convective boundary layer, heated over ~6 hours
    // ρ × c_p × h = 1.225 × 1005 × 1000 ≈ 1,231,125 J/(m²·K)
    let thermal_mass = 1.225 * 1005.0 * 1000.0;
    let heating_seconds = 6.0 * 3600.0; // 6 hours of effective heating
    let dt = net_radiation_peak * heating_seconds / thermal_mass;

    // Cloud damping: clouds reduce diurnal range
    let cf = cloud_fraction.clamp(0.0, 1.0);
    dt * (1.0 - 0.5 * cf)
}

/// Effective radiative equilibrium temperature (K).
///
/// T_eq = (S × (1 - α) / (4 × σ))^0.25
///
/// The temperature a planet would reach balancing absorbed solar and emitted
/// longwave radiation. Earth's T_eq ≈ 255 K (actual ~288 K due to greenhouse).
#[must_use]
#[inline]
pub fn equilibrium_temperature(solar_irradiance: f64, albedo: f64) -> f64 {
    let absorbed = solar_irradiance * (1.0 - albedo);
    if absorbed <= 0.0 {
        return 0.0;
    }
    (absorbed / (4.0 * STEFAN_BOLTZMANN)).powf(0.25)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- solar geometry --

    #[test]
    fn declination_summer_solstice() {
        // Day ~172 (June 21): declination should be ~+23.44°
        let d = solar_declination(172);
        assert!(
            (d - 23.44_f64.to_radians()).abs() < 2.0_f64.to_radians(),
            "summer solstice declination should be ~+23.4°, got {:.1}°",
            d.to_degrees()
        );
    }

    #[test]
    fn declination_winter_solstice() {
        // Day ~355 (Dec 21): declination should be ~-23.44°
        let d = solar_declination(355);
        assert!(
            (d - (-23.44_f64.to_radians())).abs() < 2.0_f64.to_radians(),
            "winter solstice declination should be ~-23.4°, got {:.1}°",
            d.to_degrees()
        );
    }

    #[test]
    fn declination_equinox() {
        // Day ~80 (March 21): declination should be ~0°
        let d = solar_declination(80);
        assert!(
            d.abs() < 2.0_f64.to_radians(),
            "equinox declination should be ~0°, got {:.1}°",
            d.to_degrees()
        );
    }

    #[test]
    fn hour_angle_noon() {
        assert!(hour_angle(12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn hour_angle_morning() {
        assert!(
            hour_angle(6.0) < 0.0,
            "morning hour angle should be negative"
        );
    }

    #[test]
    fn hour_angle_afternoon() {
        assert!(
            hour_angle(18.0) > 0.0,
            "afternoon hour angle should be positive"
        );
    }

    #[test]
    fn zenith_at_noon_equator_equinox() {
        let z = solar_zenith_angle(0.0, 0.0, 0.0);
        assert!(
            z.abs() < 0.01,
            "sun should be overhead at equator noon on equinox, got {:.1}°",
            z.to_degrees()
        );
    }

    #[test]
    fn zenith_below_horizon() {
        // Midnight at equator on equinox
        let z = solar_zenith_angle(0.0, 0.0, PI);
        assert!(z > PI / 2.0, "sun should be below horizon at midnight");
    }

    #[test]
    fn zenith_high_latitude_summer() {
        // 65°N, summer solstice, noon
        let lat = 65.0_f64.to_radians();
        let dec = 23.44_f64.to_radians();
        let z = solar_zenith_angle(lat, dec, 0.0);
        assert!(
            z.to_degrees() < 50.0,
            "high-latitude summer noon should have moderate zenith, got {:.1}°",
            z.to_degrees()
        );
    }

    // -- shortwave --

    #[test]
    fn clear_sky_irradiance_noon() {
        let irr = clear_sky_irradiance(0.0, CLEAR_SKY_TRANSMISSIVITY);
        assert!(
            (irr - SOLAR_CONSTANT * CLEAR_SKY_TRANSMISSIVITY).abs() < 1.0,
            "overhead sun should give S×τ, got {irr}"
        );
    }

    #[test]
    fn clear_sky_irradiance_below_horizon() {
        let irr = clear_sky_irradiance(2.0, CLEAR_SKY_TRANSMISSIVITY); // >π/2
        assert_eq!(irr, 0.0);
    }

    #[test]
    fn clear_sky_irradiance_at_angle() {
        let z = 60.0_f64.to_radians();
        let irr = clear_sky_irradiance(z, CLEAR_SKY_TRANSMISSIVITY);
        let expected = SOLAR_CONSTANT * CLEAR_SKY_TRANSMISSIVITY * 0.5; // cos(60°) = 0.5
        assert!(
            (irr - expected).abs() < 1.0,
            "60° zenith should halve irradiance, got {irr}"
        );
    }

    #[test]
    fn cloud_attenuation_clear() {
        let att = cloud_attenuated_irradiance(1000.0, 0.0);
        assert!(
            (att - 1000.0).abs() < f64::EPSILON,
            "no clouds → no attenuation"
        );
    }

    #[test]
    fn cloud_attenuation_overcast() {
        let att = cloud_attenuated_irradiance(1000.0, 1.0);
        assert!(
            att > 200.0 && att < 300.0,
            "full overcast should reduce to ~25%, got {att}"
        );
    }

    #[test]
    fn cloud_attenuation_partial() {
        let att = cloud_attenuated_irradiance(1000.0, 0.5);
        assert!(
            att > 800.0 && att < 1000.0,
            "50% cloud should reduce modestly, got {att}"
        );
    }

    // -- longwave --

    #[test]
    fn longwave_emission_earth_surface() {
        // Earth surface at 288K, emissivity ~0.95
        let lw = longwave_emission(0.95, 288.15);
        // σ × 288.15⁴ ≈ 390 W/m², ×0.95 ≈ 371
        assert!(
            lw > 350.0 && lw < 400.0,
            "Earth surface LW should be ~371 W/m², got {lw}"
        );
    }

    #[test]
    fn longwave_emission_zero_temp() {
        assert_eq!(longwave_emission(1.0, 0.0), 0.0);
    }

    #[test]
    fn atmospheric_longwave_typical() {
        let lw = atmospheric_longwave(ATMOSPHERIC_EMISSIVITY, 288.15);
        assert!(
            lw > 250.0 && lw < 350.0,
            "atmospheric LW should be ~293 W/m², got {lw}"
        );
    }

    #[test]
    fn atmospheric_longwave_zero_temp() {
        assert_eq!(atmospheric_longwave(0.75, 0.0), 0.0);
    }

    // -- net radiation --

    #[test]
    fn net_radiation_daytime_positive() {
        // Typical midday: 800 W/m² solar, 0.2 albedo, 300 LW down, 390 LW up
        let rn = net_radiation(800.0, 0.2, 300.0, 390.0);
        // (1-0.2)×800 + 300 - 390 = 640 + 300 - 390 = 550
        assert!(
            (rn - 550.0).abs() < 1.0,
            "midday net radiation should be ~550 W/m², got {rn}"
        );
    }

    #[test]
    fn net_radiation_nighttime_negative() {
        // Night: 0 solar, 280 LW down, 350 LW up
        let rn = net_radiation(0.0, 0.3, 280.0, 350.0);
        assert!(
            rn < 0.0,
            "nighttime should have negative net radiation, got {rn}"
        );
    }

    // -- diurnal --

    #[test]
    fn diurnal_range_clear_sky() {
        let dtr = diurnal_temperature_range(500.0, 0.0);
        // 500 × 21600 / 1231125 ≈ 8.8°C — realistic for clear sky over land
        assert!(
            dtr > 5.0 && dtr < 15.0,
            "clear sky DTR should be ~8-9°C, got {dtr}"
        );
    }

    #[test]
    fn diurnal_range_clouds_damp() {
        let clear = diurnal_temperature_range(500.0, 0.0);
        let cloudy = diurnal_temperature_range(500.0, 0.8);
        assert!(
            cloudy < clear,
            "clouds should reduce diurnal range: clear={clear}, cloudy={cloudy}"
        );
    }

    #[test]
    fn diurnal_range_negative_radiation() {
        assert_eq!(diurnal_temperature_range(-100.0, 0.0), 0.0);
    }

    // -- equilibrium temperature --

    #[test]
    fn equilibrium_temperature_earth() {
        let t_eq = equilibrium_temperature(SOLAR_CONSTANT, EARTH_ALBEDO);
        // Earth's radiative equilibrium ≈ 255 K
        assert!(
            (t_eq - 255.0).abs() < 3.0,
            "Earth T_eq should be ~255 K, got {t_eq}"
        );
    }

    #[test]
    fn equilibrium_temperature_no_albedo() {
        let t_eq = equilibrium_temperature(SOLAR_CONSTANT, 0.0);
        assert!(
            t_eq > 255.0,
            "zero albedo should give higher T_eq than Earth"
        );
    }

    #[test]
    fn equilibrium_temperature_full_albedo() {
        let t_eq = equilibrium_temperature(SOLAR_CONSTANT, 1.0);
        assert_eq!(t_eq, 0.0, "full albedo → no absorbed energy → 0 K");
    }

    // -- day length / sunrise-sunset --

    #[test]
    fn day_length_equinox() {
        let dl = day_length(0.0, 0.0);
        assert!(
            (dl - 12.0).abs() < 0.1,
            "equinox at equator should be ~12h, got {dl}"
        );
    }

    #[test]
    fn day_length_summer_high_latitude() {
        let dl = day_length(70.0_f64.to_radians(), 23.44_f64.to_radians());
        assert!(
            dl > 20.0,
            "high-latitude summer should have very long days, got {dl}"
        );
    }

    #[test]
    fn day_length_midnight_sun() {
        let dl = day_length(80.0_f64.to_radians(), 23.44_f64.to_radians());
        assert!((dl - 24.0).abs() < f64::EPSILON, "should be midnight sun");
    }

    #[test]
    fn day_length_polar_night() {
        let dl = day_length(80.0_f64.to_radians(), (-23.44_f64).to_radians());
        assert!(dl.abs() < f64::EPSILON, "should be polar night");
    }

    #[test]
    fn sunrise_sunset_equinox() {
        let (sr, ss) = sunrise_sunset(45.0_f64.to_radians(), 0.0);
        assert!(
            (sr - 6.0).abs() < 0.1,
            "equinox sunrise should be ~6:00, got {sr}"
        );
        assert!(
            (ss - 18.0).abs() < 0.1,
            "equinox sunset should be ~18:00, got {ss}"
        );
    }

    #[test]
    fn sunrise_sunset_symmetry() {
        let dec = 10.0_f64.to_radians();
        let lat = 40.0_f64.to_radians();
        let (sr, ss) = sunrise_sunset(lat, dec);
        assert!(
            (sr + ss - 24.0).abs() < 0.01,
            "sunrise + sunset should sum to 24h"
        );
    }
}
