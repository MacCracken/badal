//! # Badal
//!
//! **Badal** (बादल — Hindi/Urdu for "cloud") — weather and atmospheric modeling
//! engine for the AGNOS ecosystem.
//!
//! Provides ISA atmosphere, pressure systems, moisture/humidity, cloud classification,
//! wind dynamics, and atmospheric stability analysis. Built on [`hisab`] for math.

pub mod error;
pub mod atmosphere;
pub mod pressure;
pub mod moisture;
pub mod cloud;
pub mod wind;
pub mod stability;

#[cfg(feature = "logging")]
pub mod logging;

pub use error::{BadalError, Result};
pub use atmosphere::{AtmosphericState, standard_temperature, standard_pressure, air_density};
pub use moisture::saturation_vapor_pressure;
pub use wind::{coriolis_parameter, wind_chill, beaufort_scale};
pub use cloud::CloudType;
pub use stability::StabilityClass;
