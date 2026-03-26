# Changelog

## [0.1.0] - 2026-03-25

Initial scaffold with real meteorology implementations.

### Modules
- **atmosphere** — ISA standard model, dew point, air density, AtmosphericState
- **pressure** — barometric formula, pressure gradient, geostrophic wind, sea level correction
- **moisture** — saturation VP (Magnus-Tetens), mixing ratio, humidity, heat index, wet bulb
- **cloud** — 10 CloudType variants, cloud base altitude, LCL
- **wind** — Coriolis, wind chill (NWS), Beaufort scale, thermal wind shear
- **stability** — lapse rates, CAPE, lifted index, K-index, StabilityClass
