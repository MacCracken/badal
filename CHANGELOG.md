# Changelog

## [Unreleased]

## [1.0.0] - 2026-03-26

Stable release. All pre-1.0 milestones complete.

### Modules

| Module | Feature | Description |
|--------|---------|-------------|
| **atmosphere** | default | ISA model (0–20 km), AtmosphericState (validated), air density, virtual/potential temperature, pressure/density altitude |
| **pressure** | default | Barometric formula, pressure gradient force, geostrophic wind, sea level correction, altimeter setting |
| **moisture** | default | Saturation VP (Bolton 1980), dew point, mixing ratio, humidity, heat index (NWS), wet bulb (Stull) |
| **cloud** | default | 10 CloudType variants with Display, cloud base altitude, LCL, precipitation classification |
| **wind** | default | Coriolis, wind chill (NWS), Beaufort scale, thermal wind shear, wind direction/speed from u/v, log wind profile |
| **stability** | default | Dry/moist adiabatic lapse rates, CAPE, CIN, lifted index, K-index, Total Totals, Brunt-Väisälä, StabilityClass |
| **precipitation** | default | Rain rate (Cb/Ns/St + CAPE), 7 PrecipitationType variants, WMO intensity, accumulation, snow SLR, freezing level |
| **radiation** | default | Solar geometry (declination, zenith, hour angle), clear-sky + cloud irradiance (Kasten & Czeplak), longwave, net radiation, DTR, equilibrium temperature, day length, sunrise/sunset |
| **mesoscale** | default | Sea/land breeze, katabatic/anabatic winds, valley wind phase, UHI (Oke 1973), canyon SVF |
| **severe** | default | SCP (Thompson 2003), STP (Thompson 2012), DCP (Evans & Doswell), BRN, EHI, 6-tier ThreatLevel |
| **coupling** | `fluids` | Pravash integration: atmospheric grid, Coriolis/PGF forcing, flood from rainfall, wind field extraction |
| **thermal** | `thermo` | Ushma integration: surface energy balance, Penman-Monteith ET₀, sensible/latent/ground heat flux, radiative cooling |

### Science Accuracy (P-1 hardening)
- R_AIR corrected to ICAO precise value (287.052_87 J/(kg·K))
- Magnus-Tetens unified to Bolton (1980): a=17.67, b=243.5, e₀=611.2 Pa
- Heat index: full NWS Rothfusz regression with low/high humidity adjustments
- SCP/STP normalizations corrected to SPC conventions (Thompson et al.)
- Stability tolerance narrowed from 0.001 to 0.0003 °C/m
- DTR mixing layer corrected from 100m to 1000m
- Hail detection: surface wet-bulb guard removed (warm-season hail)
- Stratus added to precipitation-producing clouds
- ISA valid range documented (0–20 km)

### Stats
- 266 tests (259 unit + 7 integration), zero clippy warnings
- 27 Criterion benchmarks with CSV history
- 96.40% line coverage, 99.36% function coverage (cargo-llvm-cov)
- Zero unwrap/panic in library code
- API: `#[non_exhaustive]` on enums, `#[must_use]` on pure functions, `#[inline]` on hot paths

## [0.1.0] - 2026-03-25

Initial scaffold with real meteorology implementations.

### Modules
- **atmosphere** — ISA standard model, dew point, air density, AtmosphericState
- **pressure** — barometric formula, pressure gradient, geostrophic wind, sea level correction
- **moisture** — saturation VP (Magnus-Tetens), mixing ratio, humidity, heat index, wet bulb
- **cloud** — 10 CloudType variants, cloud base altitude, LCL
- **wind** — Coriolis, wind chill (NWS), Beaufort scale, thermal wind shear
- **stability** — lapse rates, CAPE, lifted index, K-index, StabilityClass
