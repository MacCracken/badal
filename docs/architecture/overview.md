# Badal Architecture

```
badal
├── atmosphere.rs  — ISA model, AtmosphericState, dew point, air density
├── pressure.rs    — Barometric formula, geostrophic wind, sea level correction
├── moisture.rs    — Saturation VP, mixing ratio, humidity, heat index, wet bulb
├── cloud.rs       — 10 CloudType variants, cloud base, LCL
├── wind.rs        — Coriolis, wind chill, Beaufort, thermal wind
└── stability.rs   — Lapse rates, CAPE, lifted index, K-index, StabilityClass
```

Consumers: kiran/joshua, bhava 1.5, pavan, goonj
