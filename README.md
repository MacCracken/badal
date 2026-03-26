# Badal

**Badal** (बादल — Hindi/Urdu for "cloud") — weather and atmospheric modeling engine for the [AGNOS](https://github.com/MacCracken/agnosticos) ecosystem.

## Features

- **Atmosphere** — ISA standard model, dew point, air density, atmospheric state at altitude
- **Pressure** — barometric formula, pressure gradient force, geostrophic wind, sea level correction
- **Moisture** — saturation vapor pressure (Magnus-Tetens), mixing ratio, relative/specific humidity, heat index, wet bulb
- **Clouds** — 10 cloud types, cloud base altitude, lifting condensation level
- **Wind** — Coriolis parameter, wind chill (NWS), Beaufort scale, thermal wind shear
- **Stability** — dry/moist adiabatic lapse rates, CAPE, lifted index, K-index, stability classification

## Quick Start

```rust
use badal::{atmosphere, moisture, wind};

let state = atmosphere::AtmosphericState::at_altitude(3000.0);
println!("T={:.1}K, P={:.0}Pa", state.temperature_k, state.pressure_pa);

let es = moisture::saturation_vapor_pressure(20.0); // ~2338 Pa
let b = wind::beaufort_scale(15.0); // 7 (High wind)
```

## Consumers

- **kiran/joshua** — game weather, environmental effects
- **bhava 1.5** — planetary conditions → personality modulation
- **pavan** — atmospheric state feeds aerodynamics
- **goonj** — temperature → speed of sound

## License

GPL-3.0
