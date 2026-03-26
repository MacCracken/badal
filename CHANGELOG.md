# Changelog

## [Unreleased]

### Changed
- **moisture** — unified Magnus-Tetens coefficients to Bolton (1980): a=17.67, b=243.5, e₀=611.2 Pa across both `saturation_vapor_pressure` and `dew_point`
- **moisture** — `dew_point()` moved from `atmosphere` module to `moisture` module where it belongs
- **moisture** — `dew_point()` now returns `Option<f64>`, returning `None` for invalid humidity (≤0% or >100%) instead of panicking on ln(0)
- **cloud** — added `Display` impl for `CloudType`
- **atmosphere** — added `PartialEq` derive on `AtmosphericState`
- **stability** — fixed doc comments on `StabilityClass` variants to match actual code behavior
- **pressure** — clarified `barometric_pressure` doc: isothermal approximation vs ISA

### Fixed
- **pressure** — removed unused `SEA_LEVEL_PRESSURE` import (clippy)
- **moisture** — removed unnecessary `let` binding in `heat_index` (clippy)
- Formatting issues across all source files

### Added
- Missing `#[inline]` on hot-path functions: `classify_stability`, `cape_simple`, `barometric_pressure`, `heat_index`, `wet_bulb_temperature`
- Doc comment on `logging::init()`
- Tests: stratosphere pressure, `AtmosphericState::at_altitude`, serde round-trips (AtmosphericState, CloudType, StabilityClass), `air_density` edge cases, `cape_simple` zero-temp edge, `altimeter_setting`, `geostrophic_wind_speed` zero-coriolis, `thermal_wind_shear` (positive/zero-coriolis/zero-gradient), `wet_bulb_temperature` extremes, `dew_point` validation, end-to-end weather profile integration test
- Benchmarks: `standard_pressure`, `air_density`, `dew_point`, `heat_index`, `wet_bulb_temperature`, `wind_chill`, `barometric_pressure`, `classify_stability`, `cape_simple`
- `docs/architecture/math.md` — complete mathematical reference with formulas and sources
- Tests: 45 → 79 (74 unit + 5 integration)
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
