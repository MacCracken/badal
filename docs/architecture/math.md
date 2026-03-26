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

## Precipitation

### Rain Rate — Convective (Cumulonimbus)
```
R = 2 × (CAPE / 100)^0.5   [mm/hr], capped at 100
```
Empirical parameterization; rate scales with sqrt(CAPE).

### Rain Rate — Stratiform (Nimbostratus)
```
R = 1 + 0.003 × CAPE   [mm/hr], capped at 10
```
Steadier, less CAPE-dependent than convective.

### Intensity Classification (WMO)
| Category | Rate (mm/hr) |
|----------|-------------|
| Light    | < 2.5       |
| Moderate | 2.5–7.5     |
| Heavy    | 7.5–50      |
| Violent  | > 50        |

### Accumulation
```
A = R × Δt   [mm]
```
R in mm/hr, Δt in hours.

### Snow-to-Liquid Ratio (SLR)
Temperature-dependent:
- T < -15°C: SLR = 15 (dry, fluffy)
- -15 ≤ T < -5°C: SLR = 12
- T ≥ -5°C: SLR = 8 (wet, heavy)

Snow depth (cm) = liquid_mm × SLR / 10.

### Freezing Level
```
z_freeze = T_surface / 0.0065   [m]
```
Assumes standard lapse rate (6.5°C/km). Returns 0 if surface ≤ 0°C.

## Radiation

### Solar Declination
```
δ ≈ -23.44° × cos(2π/365 × (day + 10))   [rad]
```
Approximation accurate to ±1°.

### Hour Angle
```
h = (hour - 12) × 15°   [rad]
```
Negative morning, positive afternoon, 0 at solar noon.

### Solar Zenith Angle
```
cos(θ_z) = sin(φ)sin(δ) + cos(φ)cos(δ)cos(h)
```
φ = latitude, δ = declination, h = hour angle. θ_z > 90° = below horizon.

### Clear-Sky Solar Irradiance
```
I = S × τ × cos(θ_z)   [W/m²]
```
S = 1361 W/m² (solar constant), τ = atmospheric transmissivity (~0.75 clear sky).

### Cloud Attenuation — Kasten & Czeplak (1980)
```
I_cloudy = I_clear × (1 - 0.75 × CF^3.4)
```
CF = cloud fraction [0, 1].

Source: Kasten, F. & Czeplak, G. (1980). "Solar and terrestrial radiation dependent on the amount and type of cloud." *Solar Energy*, 24(2), 177–189.

### Longwave Emission — Stefan-Boltzmann
```
F = ε × σ × T⁴   [W/m²]
```
σ = 5.670374419 × 10⁻⁸ W/(m²·K⁴), ε = emissivity.

### Net Radiation
```
R_net = (1 - α) × S_in + L_down - L_up   [W/m²]
```
α = albedo. Positive = warming, negative = cooling.

### Diurnal Temperature Range (simplified)
```
ΔT ≈ R_net × Δt / (ρ × c_p × h_mix) × (1 - 0.5 × CF)
```
ρ = 1.225 kg/m³, c_p = 1005 J/(kg·K), h_mix = 100 m, Δt = 6 hr.

### Radiative Equilibrium Temperature
```
T_eq = (S × (1 - α) / (4σ))^0.25   [K]
```
Earth: T_eq ≈ 255 K (actual ~288 K due to greenhouse effect).

## Mesoscale

### Sea/Land Breeze
```
V = √(g × h × |ΔT| / T_mean)   [m/s]
```
h = boundary layer depth, ΔT = land − sea temperature.
Positive = sea breeze (onshore), negative = land breeze (offshore).

### Sea Breeze Front Penetration
```
d = V × t × 0.5   [m]
```
Front advances at roughly half the breeze speed. Convert to km: d/1000.

### Katabatic / Anabatic Wind
```
V = √(2 × g × Δz × ΔT / T_env)   [m/s]
```
Δz = L × sin(slope), L = slope length.
Katabatic: cold air drainage downslope (night). Anabatic: warm air rising upslope (day).

### Valley Wind Phase
```
phase = -cos(2π × (hour - 3) / 24)
```
Positive = upvalley (day), negative = downvalley (night). Peak upvalley ~14:00, downvalley ~03:00.

### Urban Heat Island — Oke (1973)
```
ΔT_UHI = (pop / 10000)^0.27 × wind_factor × cloud_factor
```
- wind_factor = 1 / (1 + 0.3V) for V > 0.5 m/s
- cloud_factor = 1 - 0.6 × CF

Source: Oke, T.R. (1973). "City size and the urban heat island." *Atmospheric Environment*, 7(8), 769–779.

### Canyon Temperature Excess
```
ΔT_canyon = ΔT_UHI × (1 + (1 - SVF))
```
SVF = sky view factor [0, 1]. Lower SVF (narrower canyon) amplifies UHI up to 2×.

## Severe Weather

### Supercell Composite Parameter (SCP)
```
SCP = (CAPE / 1000) × (shear_0-6km / 20) × (SRH / 100)
```
SCP > 1 favors supercells. SCP > 4 favors significant supercells.

### Significant Tornado Parameter (STP)
```
STP = (CAPE / 1500) × (shear / 20) × (SRH / 150) × ((2000 - LCL) / 1000)
```
LCL term clamped [0, 2]. STP > 1 favors significant (EF2+) tornadoes.

### Derecho Composite Parameter (DCP)
```
DCP = (CAPE / 980) × (shear / 18) × (mean_wind / 16)
```
DCP > 2 favors derecho development.

### Bulk Richardson Number (BRN)
```
BRN = CAPE / (0.5 × shear²)
```
BRN 10–45: supercells. < 10: too much shear. > 45: multicell (insufficient shear).

### Energy-Helicity Index (EHI)
```
EHI = (CAPE × SRH) / 160000
```
EHI > 1: tornado possible. EHI > 2: significant tornado likely.
