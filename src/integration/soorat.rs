//! Soorat integration — visualization data structures for atmospheric modeling.
//!
//! Provides structured types that soorat can render: cloud fields, wind vectors,
//! precipitation maps, and atmospheric cross-sections.

use serde::{Deserialize, Serialize};

// ── Cloud field visualization ──────────────────────────────────────────────

/// Cloud layer data for volumetric/billboard rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudField {
    /// Cloud layers from lowest to highest.
    pub layers: Vec<CloudLayer>,
}

/// A single cloud layer.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CloudLayer {
    /// Base altitude (m).
    pub base_altitude_m: f32,
    /// Top altitude (m).
    pub top_altitude_m: f32,
    /// Coverage fraction (0.0–1.0).
    pub coverage: f32,
    /// Cloud type index (maps to CloudType enum ordinal).
    pub cloud_type: u8,
    /// Optical thickness (0 = transparent, >3 = opaque).
    pub optical_depth: f32,
}

impl CloudField {
    /// Create a simple single-layer cloud field.
    #[must_use]
    pub fn single_layer(base_m: f32, thickness_m: f32, coverage: f32, cloud_type: u8) -> Self {
        Self {
            layers: vec![CloudLayer {
                base_altitude_m: base_m,
                top_altitude_m: base_m + thickness_m,
                coverage: coverage.clamp(0.0, 1.0),
                cloud_type,
                optical_depth: coverage * 5.0,
            }],
        }
    }
}

// ── Wind vector field ──────────────────────────────────────────────────────

/// A 2D or 3D grid of wind velocity vectors for arrow/streamline rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindVectorField {
    /// Wind velocity at each grid point `[u, v, w]` in m/s.
    /// Flattened: `vectors[z * ny * nx + y * nx + x]`.
    pub vectors: Vec<[f32; 3]>,
    /// Grid dimensions (nx, ny, nz). For 2D surface wind, nz = 1.
    pub dimensions: [usize; 3],
    /// World-space origin `[x, y, z]` in metres.
    pub origin: [f32; 3],
    /// Grid spacing in metres (horizontal).
    pub spacing_h: f32,
    /// Vertical spacing in metres (between levels).
    pub spacing_v: f32,
    /// Maximum wind speed in the field (m/s).
    pub max_speed: f32,
}

impl WindVectorField {
    /// Create a uniform surface wind field (2D, nz=1).
    #[must_use]
    pub fn uniform_surface(
        nx: usize,
        ny: usize,
        origin: [f32; 3],
        spacing: f32,
        wind_u: f32,
        wind_v: f32,
    ) -> Self {
        let count = nx * ny;
        let vectors = vec![[wind_u, wind_v, 0.0]; count];
        let speed = (wind_u * wind_u + wind_v * wind_v).sqrt();
        Self {
            vectors,
            dimensions: [nx, ny, 1],
            origin,
            spacing_h: spacing,
            spacing_v: 0.0,
            max_speed: speed,
        }
    }
}

// ── Precipitation map ──────────────────────────────────────────────────────

/// Precipitation intensity grid for particle system seeding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrecipitationMap {
    /// Precipitation rate at each grid point (mm/hr).
    /// Flattened row-major: `rates[y * nx + x]`.
    pub rates: Vec<f32>,
    /// Grid dimensions (nx, ny).
    pub dimensions: [usize; 2],
    /// World-space origin `[x, z]` in metres (horizontal plane).
    pub origin: [f32; 2],
    /// Grid spacing in metres.
    pub spacing: f32,
    /// Precipitation type (0 = rain, 1 = snow, 2 = sleet, 3 = hail).
    pub precip_type: u8,
    /// Maximum rate in the field (mm/hr).
    pub max_rate: f32,
}

impl PrecipitationMap {
    /// Create a uniform precipitation field.
    #[must_use]
    pub fn uniform(
        nx: usize,
        ny: usize,
        origin: [f32; 2],
        spacing: f32,
        rate_mm_hr: f32,
        precip_type: u8,
    ) -> Self {
        let count = nx * ny;
        Self {
            rates: vec![rate_mm_hr; count],
            dimensions: [nx, ny],
            origin,
            spacing,
            precip_type,
            max_rate: rate_mm_hr,
        }
    }
}

// ── Atmospheric cross-section ──────────────────────────────────────────────

/// Atmospheric cross-section data for contour/isobar rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtmosphericSection {
    /// Temperature values (K) at grid points.
    /// Flattened: `values[altitude_idx * nx + x_idx]`.
    pub temperature: Vec<f32>,
    /// Pressure values (Pa) at grid points.
    pub pressure: Vec<f32>,
    /// Grid dimensions (nx_horizontal, ny_altitude).
    pub dimensions: [usize; 2],
    /// Horizontal range (min_x, max_x) in metres.
    pub horizontal_range: [f32; 2],
    /// Altitude range (min_alt, max_alt) in metres.
    pub altitude_range: [f32; 2],
}

impl AtmosphericSection {
    /// Create an ISA standard atmosphere cross-section.
    ///
    /// `nx`: horizontal samples, `n_alt`: altitude samples.
    /// `max_altitude_m`: top of the section.
    #[must_use]
    pub fn isa_section(x_range: [f32; 2], nx: usize, max_altitude_m: f32, n_alt: usize) -> Self {
        let count = nx * n_alt;
        let mut temperature = Vec::with_capacity(count);
        let mut pressure = Vec::with_capacity(count);

        let alt_step = if n_alt > 1 {
            max_altitude_m / (n_alt - 1) as f32
        } else {
            0.0
        };

        for iy in 0..n_alt {
            let alt = iy as f32 * alt_step;
            let t = crate::atmosphere::standard_temperature(alt as f64) as f32;
            let p = crate::atmosphere::standard_pressure(alt as f64) as f32;
            for _ in 0..nx {
                temperature.push(t);
                pressure.push(p);
            }
        }

        Self {
            temperature,
            pressure,
            dimensions: [nx, n_alt],
            horizontal_range: x_range,
            altitude_range: [0.0, max_altitude_m],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cloud_field_single_layer() {
        let field = CloudField::single_layer(2000.0, 500.0, 0.7, 3);
        assert_eq!(field.layers.len(), 1);
        assert!((field.layers[0].base_altitude_m - 2000.0).abs() < 0.1);
        assert!((field.layers[0].top_altitude_m - 2500.0).abs() < 0.1);
        assert!((field.layers[0].coverage - 0.7).abs() < 0.01);
    }

    #[test]
    fn cloud_field_coverage_clamps() {
        let field = CloudField::single_layer(1000.0, 200.0, 1.5, 0);
        assert!((field.layers[0].coverage - 1.0).abs() < 0.01);
    }

    #[test]
    fn wind_field_uniform() {
        let field = WindVectorField::uniform_surface(4, 4, [0.0; 3], 1000.0, 5.0, 3.0);
        assert_eq!(field.vectors.len(), 16);
        assert_eq!(field.dimensions, [4, 4, 1]);
        for v in &field.vectors {
            assert_eq!(v[0], 5.0);
            assert_eq!(v[1], 3.0);
            assert_eq!(v[2], 0.0);
        }
        assert!((field.max_speed - (5.0_f32.powi(2) + 3.0_f32.powi(2)).sqrt()).abs() < 0.01);
    }

    #[test]
    fn precipitation_map_uniform() {
        let map = PrecipitationMap::uniform(3, 3, [0.0, 0.0], 500.0, 10.0, 0);
        assert_eq!(map.rates.len(), 9);
        assert_eq!(map.precip_type, 0);
        assert!((map.max_rate - 10.0).abs() < 0.01);
    }

    #[test]
    fn atmospheric_section_isa() {
        let section = AtmosphericSection::isa_section([0.0, 10000.0], 5, 11000.0, 12);
        assert_eq!(section.temperature.len(), 60); // 5 × 12
        assert_eq!(section.pressure.len(), 60);
        // Sea level should be ~288K, ~101325 Pa
        assert!((section.temperature[0] - 288.15).abs() < 0.1);
        assert!((section.pressure[0] - 101325.0).abs() < 1.0);
        // 11km should be much colder
        let top_idx = 11 * 5; // last altitude row
        assert!(section.temperature[top_idx] < 230.0);
    }

    #[test]
    fn atmospheric_section_single_level() {
        let section = AtmosphericSection::isa_section([0.0, 1000.0], 3, 0.0, 1);
        assert_eq!(section.temperature.len(), 3);
    }

    #[test]
    fn cloud_field_serializes() {
        let field = CloudField::single_layer(3000.0, 1000.0, 0.5, 2);
        let json = serde_json::to_string(&field);
        assert!(json.is_ok());
    }

    #[test]
    fn wind_field_serializes() {
        let field = WindVectorField::uniform_surface(2, 2, [0.0; 3], 100.0, 1.0, 0.0);
        let json = serde_json::to_string(&field);
        assert!(json.is_ok());
    }
}
