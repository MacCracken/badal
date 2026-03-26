# Mathematical Reference

Formulas and coefficients used in Badal, with sources.

## Atmosphere

### ISA Standard Temperature (troposphere, h ≤ 11000m)
```
T(h) = 288.15 - 0.0065 × h   [K]
```
Above 11000m (tropopause): T = 216.65 K (constant).

Source: ICAO Standard Atmosphere (Doc 7488/3).

### ISA Standard Pressure
Troposphere (h ≤ 11000m):
```
P(h) = 101325 × (T(h) / 288.15) ^ (g / (L × R))
```
where L = 0.0065 K/m, R = 287.058 J/(kg·K), g = 9.80665 m/s².

Stratosphere (h > 11000m, isothermal):
```
P(h) = P(11000) × exp(-g × (h - 11000) / (R × 216.65))
```

### Air Density (Ideal Gas Law)
```
ρ = P / (R × T)   [kg/m³]
```

## Moisture

### Saturation Vapor Pressure — Magnus-Tetens (Bolton 1980)
```
e_s(T) = 611.2 × exp(17.67 × T / (T + 243.5))   [Pa]
```
T in °C. Coefficients: a = 17.67, b = 243.5, e_s0 = 611.2 Pa.

Source: Bolton, D. (1980). "The Computation of Equivalent Potential Temperature." *Monthly Weather Review*, 108(7), 1046–1053.

### Dew Point (inverse Magnus-Tetens)
```
γ = a × T / (b + T) + ln(RH / 100)
T_d = b × γ / (a - γ)   [°C]
```
Uses same Bolton coefficients as saturation VP for consistency.

### Mixing Ratio
```
w = 0.622 × e / (P - e)   [kg/kg]
```

### Specific Humidity
```
q = 0.622 × e / (P - 0.378 × e)   [kg/kg]
```

### Heat Index — Rothfusz Regression
```
HI = -8.784695 + 1.61139411T + 2.338549R - 0.14611605TR
     - 0.012308094T² - 0.016424828R² + 0.002211732T²R
     + 0.00072546TR² - 0.000003582T²R²
```
Valid for T > 27°C and RH > 40%. T in °C, R = RH in %.

Source: Rothfusz, L.P. (1990). NWS Southern Region Technical Attachment SR 90-23.

### Wet Bulb Temperature — Stull Formula
```
Tw ≈ T × atan(0.151977 × (RH + 8.313659)^0.5)
     + atan(T + RH) - atan(RH - 1.676331)
     + 0.00391838 × RH^1.5 × atan(0.023101 × RH) - 4.686035
```

Source: Stull, R. (2011). "Wet-Bulb Temperature from Relative Humidity and Air Temperature." *J. Appl. Meteor. Climatol.*, 50(11), 2267–2269.

## Pressure

### Barometric Formula (isothermal)
```
P(h) = P_0 × exp(-g × h / (R × T))
```
Assumes constant T through the layer. Less accurate than ISA for large altitude ranges.

### Sea Level Correction
```
P_sl = P_station × exp(g × h / (R × T))
```

### Pressure Gradient Force
```
F/m = -(1/ρ) × (dP/dx)   [m/s²]
```

### Geostrophic Wind Speed
```
V_g = |dP/dx| / (ρ × |f|)   [m/s]
```

## Wind

### Coriolis Parameter
```
f = 2Ω × sin(φ)   [s⁻¹]
```
Ω = 7.292 × 10⁻⁵ rad/s (Earth's angular velocity), φ = latitude in radians.

Reference values: f(45°) ≈ 1.031 × 10⁻⁴, f(90°) ≈ 1.458 × 10⁻⁴.

### Wind Chill — NWS Formula
```
WC = 13.12 + 0.6215T - 11.37V^0.16 + 0.3965TV^0.16
```
T in °C, V in km/h. Valid for T ≤ 10°C and V ≥ 4.8 km/h.

Source: NWS/Environment Canada joint formula (2001).

### Beaufort Scale
Wind speed thresholds (m/s): 0.5, 1.6, 3.4, 5.5, 8.0, 10.8, 13.9, 17.2, 20.8, 24.5, 28.5, 32.7.

### Thermal Wind Shear (simplified)
```
ΔV/Δz ≈ (g / (f × T)) × |dT/dx| × 1000   [m/s per 1000m]
```
Using T ≈ 280 K as reference temperature.

## Stability

### Lapse Rates
- Dry adiabatic: Γ_d = 9.8 °C/km = 0.0098 °C/m
- Moist adiabatic: Γ_m ≈ 6 °C/km = 0.006 °C/m (varies with temperature)

### CAPE (simplified single-layer)
```
CAPE = g × ((T_parcel - T_env) / T_env) × Δz   [J/kg]
```

### Lifted Index
```
LI = T_env(500mb) - T_parcel(500mb)   [°C]
```
LI > 0 → stable, LI < 0 → unstable.

### K-Index
```
KI = (T_850 - T_500) + Td_850 - (T_700 - Td_700)
```
KI > 30 → high thunderstorm risk.

## Cloud

### Cloud Base / LCL Approximation
```
z_base ≈ (T - T_d) / 8 × 1000   [m]
```
≈ 125m per °C of temperature-dew point spread.
