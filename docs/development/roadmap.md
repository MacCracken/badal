# Badal Roadmap

## Status
**v0.1.0** — Initial scaffold with real meteorology.

## Future Features (demand-gated)

### Precipitation
- Rain rate estimation from cloud type + CAPE
- Snow/hail prediction from temperature profile
- Precipitation accumulation model

### Radiation Budget
- Solar radiation at surface (clear sky + cloud attenuation)
- Longwave radiation (Stefan-Boltzmann + atmospheric window)
- Diurnal temperature cycle from radiation balance

### Mesoscale
- Sea/land breeze model
- Mountain/valley wind (katabatic/anabatic)
- Urban heat island effect

### Severe Weather
- Supercell thunderstorm parameter (SCP)
- Significant tornado parameter (STP)
- Derecho composite parameter

### Integration
- ushma coupling: surface energy balance, evapotranspiration
- pravash coupling: atmospheric flow simulation
- bhava 1.5: weather conditions → agent mood/behavior modulation

## v1.0.0 Criteria
- API frozen, zero unwrap/panic, 90%+ coverage, benchmark golden numbers
