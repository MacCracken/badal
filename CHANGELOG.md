# Changelog

## [Unreleased]

### Added
- **precipitation** — new module: `PrecipitationType` enum (None/Drizzle/Rain/Snow/Sleet/FreezingRain/Hail), `Intensity` enum (None/Light/Moderate/Heavy/Violent), `rain_rate()` from cloud type + CAPE, `precipitation_type()` from surface conditions, `classify_intensity()` (WMO thresholds), `accumulation()`, `snow_accumulation()` with temperature-dependent SLR, `snow_liquid_ratio()`, `freezing_level()`
- **radiation** — new module: solar geometry (`solar_declination`, `hour_angle`, `solar_zenith_angle`), shortwave (`clear_sky_irradiance`, `cloud_attenuated_irradiance` via Kasten & Czeplak 1980), longwave (`longwave_emission`, `atmospheric_longwave`), energy balance (`net_radiation`, `diurnal_temperature_range`, `equilibrium_temperature`), constants (`SOLAR_CONSTANT`, `STEFAN_BOLTZMANN`, `EARTH_ALBEDO`, `CLEAR_SKY_TRANSMISSIVITY`, `ATMOSPHERIC_EMISSIVITY`)
- **mesoscale** — new module: `sea_land_breeze()` thermal circulation, `sea_breeze_penetration_km()`, `katabatic_wind()` / `anabatic_wind()` slope flows, `valley_wind_phase()` diurnal cycle, `urban_heat_island()` Oke (1973) with wind/cloud modifiers, `canyon_temperature_excess()` sky view factor amplification
- **severe** — new module: `supercell_composite()` (SCP), `significant_tornado()` (STP), `derecho_composite()` (DCP), `bulk_richardson_number()` (BRN), `energy_helicity_index()` (EHI), `ThreatLevel` enum (None/Marginal/Slight/Enhanced/Moderate/High) with `classify_threat()`
- **atmosphere** — `virtual_temperature()`, `potential_temperature()`, `pressure_altitude()` (inverse ISA), `density_altitude()`
- **radiation** — `day_length()`, `sunrise_sunset()` from latitude + declination
- **stability** — `total_totals()` index, `cin_simple()` convective inhibition, `moist_adiabatic_lapse_rate()` as f(T,P), `brunt_vaisala_squared()` static stability
- **wind** — `wind_direction()` from u/v (meteorological convention), `wind_speed()` from u/v, `log_wind_profile()` height extrapolation
- **precipitation** — Stratus drizzle rain rate (0.3 mm/hr)
- Tests: 86 → 235 (228 unit + 7 integration), benchmarks: 14 → 27

### Changed
- **atmosphere** — `R_AIR` corrected from 287.058 to 287.052_87 (precise ICAO value)
- **atmosphere** — documented ISA valid range (0–20 km) on `standard_temperature` and `standard_pressure`
- **atmosphere** — `AtmosphericState` fields are now private; use `new()` for validated construction or getters (`temperature_k()`, `pressure_pa()`, `humidity_percent()`, `altitude_m()`) for access
- **moisture** — unified Magnus-Tetens coefficients to Bolton (1980): a=17.67, b=243.5, e₀=611.2 Pa across both `saturation_vapor_pressure` and `dew_point`
- **moisture** — `heat_index()` now implements full NWS algorithm with low-humidity and high-humidity adjustment terms; RH<40% guard removed (was incorrectly skipping valid heat index conditions)
- **moisture** — `dew_point()` moved from `atmosphere` module to `moisture` module where it belongs
- **moisture** — `dew_point()` now returns `Option<f64>`, returning `None` for invalid humidity (≤0% or >100%) instead of panicking on ln(0)
- **cloud** — added `Display` impl for `CloudType`
- **atmosphere** — added `PartialEq` derive on `AtmosphericState`
- **stability** — fixed doc comments on `StabilityClass` variants to match actual code behavior
- **pressure** — clarified `barometric_pressure` doc: isothermal approximation vs ISA
- **logging** — `init()` now uses `try_init()` — safe to call multiple times without panicking

### Fixed
- **stability** — neutral classification tolerance narrowed from 0.001 to 0.0003 °C/m (was classifying superadiabatic lapse rates as neutral)
- **radiation** — DTR mixing layer corrected from 100m to 1000m (was overestimating diurnal range ~8x)
- **precipitation** — hail detection no longer requires surface wet-bulb ≤ 0°C (hail commonly falls in warm conditions)
- **cloud** — `produces_precipitation()` now includes Stratus (drizzle producer)
- **severe** — SCP normalization corrected to SPC conventions: SRH/50 (was /100), shear/40 (was /20)
- **severe** — STP shear normalization corrected: BWD/12 (was /20) per Thompson et al. (2012)
- **pressure** — doc comment corrected: "hypsometric equation" → "exponential barometric formula"
- **mesoscale** — valley wind phase comment: peak corrected from 14:00 to 15:00
- **pressure** — removed unused `SEA_LEVEL_PRESSURE` import (clippy)
- **moisture** — removed unnecessary `let` binding in `heat_index` (clippy)
- Formatting issues across all source files

### Added
- **atmosphere** — `AtmosphericState::new()` validated constructor rejecting invalid temperature (≤0K), pressure (<0), humidity (outside [0,100])
- Missing `#[inline]` on hot-path functions: `classify_stability`, `cape_simple`, `barometric_pressure`, `heat_index`, `wet_bulb_temperature`
- Doc comment on `logging::init()`
- Tests: constructor validation, stratosphere pressure, `AtmosphericState::at_altitude`, serde round-trips (AtmosphericState, CloudType, StabilityClass), `air_density` edge cases, `cape_simple` zero-temp edge, `altimeter_setting`, `geostrophic_wind_speed` zero-coriolis, `thermal_wind_shear` (positive/zero-coriolis/zero-gradient), `wet_bulb_temperature` extremes, `dew_point` validation, end-to-end weather profile integration test
- Benchmarks: `standard_pressure`, `air_density`, `dew_point`, `heat_index`, `wet_bulb_temperature`, `wind_chill`, `barometric_pressure`, `classify_stability`, `cape_simple`
- `docs/architecture/math.md` — complete mathematical reference with formulas and sources
- Tests: 45 → 86 (79 unit + 7 integration), coverage 95.18%
- Benchmarks: 5 → 14

## [0.1.0] - 2026-03-25

Initial scaffold with real meteorology implementations.

### Modules
- **atmosphere** — ISA standard model, dew point, air density, AtmosphericState
- **pressure** — barometric formula, pressure gradient, geostrophic wind, sea level correction
- **moisture** — saturation VP (Magnus-Tetens), mixing ratio, humidity, heat index, wet bulb
- **cloud** — 10 CloudType variants, cloud base altitude, LCL
- **wind** — Coriolis, wind chill (NWS), Beaufort scale, thermal wind shear
- **stability** — lapse rates, CAPE, lifted index, K-index, StabilityClass
