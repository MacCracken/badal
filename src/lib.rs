//! # Badal
//!
//! **Badal** (बादल — Hindi/Urdu for "cloud") — weather and atmospheric modeling
//! engine for the AGNOS ecosystem.
//!
//! Provides ISA atmosphere, pressure systems, moisture/humidity, cloud classification,
//! wind dynamics, atmospheric stability, precipitation, radiation budget, and
//! mesoscale phenomena. Built on [`hisab`] for math.

pub mod atmosphere;
/// Cross-crate bridges — primitive-value conversions from other AGNOS science crates.
pub mod bridge;
pub mod cloud;
pub mod error;
/// Integration APIs for downstream consumers (soorat rendering).
pub mod integration;
pub mod mesoscale;
pub mod moisture;
pub mod precipitation;
pub mod pressure;
pub mod radiation;
pub mod severe;
pub mod stability;
pub mod wind;

#[cfg(feature = "fluids")]
pub mod coupling;

#[cfg(feature = "thermo")]
pub mod thermal;

#[cfg(feature = "logging")]
pub mod logging;

pub use atmosphere::{AtmosphericState, air_density, standard_pressure, standard_temperature};
pub use cloud::CloudType;
pub use error::{BadalError, Result};
pub use moisture::{dew_point, saturation_vapor_pressure};
pub use precipitation::PrecipitationType;
pub use severe::ThreatLevel;
pub use stability::StabilityClass;
pub use wind::{beaufort_scale, coriolis_parameter, wind_chill};
