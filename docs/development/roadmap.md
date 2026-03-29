# Badal Roadmap

## V1.0.0 — Stable Release (2026-03-26)

### Modules
| Module | Feature | Description |
|--------|---------|-------------|
| atmosphere | default | ISA model, AtmosphericState, virtual/potential temp, pressure/density altitude |
| pressure | default | Barometric formula, PGF, geostrophic wind, sea level correction |
| moisture | default | Saturation VP, dew point, humidity, heat index (NWS), wet bulb (Stull) |
| cloud | default | 10 cloud types, cloud base, LCL, precipitation classification |
| wind | default | Coriolis, wind chill, Beaufort, thermal wind, wind direction, log profile |
| stability | default | CAPE, CIN, lapse rates, lifted/K/TT indices, Brunt-Väisälä |
| precipitation | default | Rain rate, 7 precip types, WMO intensity, snow SLR, freezing level |
| radiation | default | Solar geometry, irradiance, longwave, net radiation, DTR, day length |
| mesoscale | default | Sea/land breeze, katabatic/anabatic, valley wind, UHI, canyon SVF |
| severe | default | SCP, STP, DCP, BRN, EHI, ThreatLevel |
| coupling | `fluids` | Pravash atmospheric grid, Coriolis/PGF forcing, flood modeling |
| thermal | `thermo` | Ushma surface energy balance, ET₀, heat fluxes, radiative cooling |

### Stats
- 266 tests (259 unit + 7 integration)
- 27 Criterion benchmarks
- 96.40% line coverage
- Zero clippy warnings, cargo audit clean, cargo deny clean
- Consumer smoke tests for kiran, joshua, pavan, goonj

## 1.1.0 — Demand-gated extensions

### Integration
- [ ] bhava 1.5: weather conditions → agent mood/behavior modulation

### Cross-Crate Bridges

- [ ] **`bridge.rs` module** — primitive-value conversions for cross-crate weather/atmospheric data
- [ ] **pavan bridge**: altitude (m) → air density (kg/m³), temperature (K), pressure (Pa); wind profile → free-stream velocity
- [ ] **goonj bridge**: temperature (°C), humidity (%) → speed of sound, air absorption coefficients
- [ ] **garjan bridge**: rain rate (mm/hr), wind speed (m/s) → already consumed by garjan's bridge (document the interface)
- [ ] **prakash bridge**: atmospheric density profile → Rayleigh scattering optical depth; cloud cover fraction → diffuse/direct light ratio
- [ ] **vanaspati bridge**: temperature (°C), rainfall (mm), solar radiation (W/m²) → growing conditions parameters

### Soorat Integration (rendering visualization)

- [ ] **`integration/soorat.rs` module** — feature-gated `soorat-compat`
- [ ] **Cloud field visualization**: cloud type, coverage, altitude layers as volumetric data for cloud rendering
- [ ] **Wind vector field**: 2D/3D wind velocity grid for arrow/streamline visualization
- [ ] **Precipitation map**: rain/snow intensity grid for particle system rendering
- [ ] **Temperature/pressure isobar map**: atmospheric cross-section data for contour rendering

### Watch List
| Item | Area |
|------|------|
| ISA layers above 20 km (stratosphere/mesosphere) | atmosphere |
| Equivalent potential temperature (theta-e) | atmosphere |
| Ice saturation / frost point | moisture |
| Foehn/chinook winds | mesoscale |
| SHIP (significant hail parameter) | severe |
| Equation of time (clock → solar time) | radiation |
| Spencer (1971) higher-accuracy declination | radiation |
