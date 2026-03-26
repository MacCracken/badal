//! # Badal
//!
//! **Badal** (बादल — Hindi/Urdu for "cloud") — weather and atmospheric modeling
//! engine for the AGNOS ecosystem.
//!
//! Provides ISA atmosphere, pressure systems, moisture/humidity, cloud classification,
//! wind dynamics, and atmospheric stability analysis. Built on [`hisab`] for math.

pub mod atmosphere;
pub mod cloud;
pub mod error;
pub mod moisture;
pub mod pressure;
pub mod stability;
pub mod wind;

#[cfg(feature = "logging")]
pub mod logging;

pub use atmosphere::{AtmosphericState, air_density, standard_pressure, standard_temperature};
pub use cloud::CloudType;
pub use error::{BadalError, Result};
pub use moisture::{dew_point, saturation_vapor_pressure};
pub use stability::StabilityClass;
pub use wind::{beaufort_scale, coriolis_parameter, wind_chill};
