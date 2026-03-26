# Badal

> **Badal** (Hindi/Urdu: बादल — cloud) — weather and atmospheric modeling engine for [AGNOS](https://github.com/MacCracken/agnosticos)

Complete meteorology library: ISA atmosphere, pressure systems, moisture/humidity, cloud classification, wind dynamics, atmospheric stability, precipitation, radiation budget, mesoscale phenomena, severe weather indices, and integration with [pravash](https://github.com/MacCracken/pravash) (fluid dynamics) and [ushma](https://github.com/MacCracken/ushma) (thermodynamics). Built on [hisab](https://github.com/MacCracken/hisab) for math.

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `atmosphere` | always | ISA model, AtmosphericState, virtual/potential temp, pressure/density altitude |
| `pressure` | always | Barometric formula, PGF, geostrophic wind, sea level correction |
| `moisture` | always | Saturation VP (Bolton), dew point, humidity, heat index (NWS), wet bulb |
| `cloud` | always | 10 CloudType variants, cloud base, LCL |
| `wind` | always | Coriolis, wind chill, Beaufort, thermal wind, direction, log profile |
| `stability` | always | CAPE, CIN, lapse rates, lifted/K/TT indices, Brunt-Väisälä |
| `precipitation` | always | Rain rate, 7 precip types, WMO intensity, snow SLR, freezing level |
| `radiation` | always | Solar geometry, irradiance, longwave, net radiation, DTR, day length |
| `mesoscale` | always | Sea/land breeze, katabatic/anabatic, valley wind, UHI, canyon |
| `severe` | always | SCP, STP, DCP, BRN, EHI, ThreatLevel |
| `fluids` | opt-in | Pravash coupling: atmospheric grid, Coriolis/PGF forcing, flood modeling |
| `thermo` | opt-in | Ushma coupling: surface energy balance, ET₀, heat fluxes |
| `logging` | opt-in | Structured tracing via `BADAL_LOG` env var |

## Quick Start

```toml
[dependencies]
badal = "1"
```

```rust
use badal::{atmosphere, moisture, wind, precipitation, radiation};

let state = atmosphere::AtmosphericState::at_altitude(3000.0);
println!("T={:.1}K, P={:.0}Pa", state.temperature_k(), state.pressure_pa());

let dp = moisture::dew_point(25.0, 60.0).unwrap();
let rate = precipitation::rain_rate(badal::CloudType::Cumulonimbus, 2000.0);
let day = radiation::day_length(45.0_f64.to_radians(), radiation::solar_declination(172));
let b = wind::beaufort_scale(15.0); // 7 (High wind)
```

## Building

```sh
cargo build
cargo test --all-features   # 266 tests
make bench                  # 27 criterion benchmarks
```

## Consumers

- **kiran/joshua** — game weather, environmental effects
- **bhava 1.5** — planetary conditions → personality modulation
- **pavan** — atmospheric state feeds aerodynamics
- **goonj** — temperature → speed of sound

## License

GPL-3.0-only — see [LICENSE](LICENSE).
