# Badal Roadmap

## Status
**v0.1.0** — Initial scaffold with full meteorology suite.

## Completed

### Precipitation
- Rain rate estimation from cloud type + CAPE
- Snow/hail prediction from temperature profile (wet bulb discriminator, SLR)
- Precipitation accumulation model (liquid + snow)
- Precipitation type classification (7 types)
- Intensity classification (WMO thresholds)
- Freezing level estimation

### Radiation Budget
- Solar geometry (declination, hour angle, zenith angle)
- Solar radiation at surface (clear sky + Kasten & Czeplak cloud attenuation)
- Longwave radiation (Stefan-Boltzmann emission + atmospheric downwelling)
- Net radiation balance
- Diurnal temperature range from energy balance
- Radiative equilibrium temperature

### Mesoscale
- Sea/land breeze model with front penetration
- Mountain/valley wind (katabatic/anabatic) with diurnal phase
- Urban heat island (Oke 1973) with wind/cloud modifiers + canyon amplification

### Severe Weather
- Supercell Composite Parameter (SCP)
- Significant Tornado Parameter (STP)
- Derecho Composite Parameter (DCP)
- Bulk Richardson Number (BRN)
- Energy-Helicity Index (EHI)
- ThreatLevel classification (6-tier)

## Future Features (demand-gated)

### Integration
- ushma coupling: surface energy balance, evapotranspiration
- pravash coupling: atmospheric flow simulation
- bhava 1.5: weather conditions → agent mood/behavior modulation

## v1.0.0 Criteria
- API frozen, zero unwrap/panic, 90%+ coverage, benchmark golden numbers
