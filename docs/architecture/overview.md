# Badal Architecture

```
badal
├── atmosphere.rs     — ISA model, AtmosphericState, air density
├── pressure.rs       — Barometric formula, geostrophic wind, sea level correction
├── moisture.rs       — Saturation VP, dew point, mixing ratio, humidity, heat index, wet bulb
├── cloud.rs          — 10 CloudType variants, cloud base, LCL
├── wind.rs           — Coriolis, wind chill, Beaufort, thermal wind
├── stability.rs      — Lapse rates, CAPE, lifted index, K-index, StabilityClass
├── precipitation.rs  — Rain rate, precip type, intensity, accumulation, snow, freezing level
├── radiation.rs      — Solar geometry, irradiance, longwave, net radiation, diurnal cycle
├── mesoscale.rs      — Sea/land breeze, katabatic/anabatic winds, valley wind, UHI, canyon
├── severe.rs         — SCP, STP, DCP, BRN, EHI, ThreatLevel classification
├── coupling.rs       — Pravash integration: atmospheric grid, Coriolis/PGF forcing, flood modeling (feat: fluids)
└── thermal.rs        — Ushma integration: surface energy balance, evapotranspiration, heat fluxes (feat: thermo)
```

Consumers: kiran/joshua, bhava 1.5, pavan, goonj
