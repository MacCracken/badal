//! Coupling between badal atmospheric state and pravash fluid solvers.
//!
//! Provides initialization, forcing, and conversion functions to drive
//! pravash grid-based Navier-Stokes and shallow water simulations with
//! atmospheric conditions from badal.
//!
//! Requires the `fluids` feature.

use crate::atmosphere::{self, AtmosphericState};
use crate::error::{BadalError, Result};
use pravash::grid::{BoundaryCondition, FluidGrid, GridConfig};
use pravash::shallow::ShallowWater;

/// Configuration for atmospheric flow simulation on a pravash grid.
#[derive(Debug, Clone, Copy)]
pub struct AtmoFlowConfig {
    /// Grid cell size in meters. Default: 1000.0 (1 km).
    pub cell_size_m: f64,
    /// Simulation timestep in seconds. Default: 1.0.
    pub dt: f64,
    /// Kinematic viscosity (m²/s). Default: 1.5e-5 (air at 20°C).
    pub viscosity: f64,
    /// Vorticity confinement strength. Default: 0.3.
    pub vorticity_confinement: f64,
    /// Smagorinsky turbulence coefficient. Default: 0.15.
    pub smagorinsky_cs: f64,
}

impl Default for AtmoFlowConfig {
    fn default() -> Self {
        Self {
            cell_size_m: 1000.0,
            dt: 1.0,
            viscosity: 1.5e-5,
            vorticity_confinement: 0.3,
            smagorinsky_cs: 0.15,
        }
    }
}

/// Create a pravash [`GridConfig`] from atmospheric flow configuration.
///
/// Sets up buoyancy-driven flow using the atmospheric state's temperature
/// as the reference, suitable for simulating thermal convection and wind.
#[must_use]
pub fn grid_config(state: &AtmosphericState, config: &AtmoFlowConfig) -> GridConfig {
    let mut gc = GridConfig::default();
    gc.dt = config.dt;
    gc.viscosity = config.viscosity;
    gc.boundary = BoundaryCondition::FreeSlip;
    gc.vorticity_confinement = config.vorticity_confinement;
    gc.buoyancy_alpha = -0.01; // negative: hot (low density) rises
    gc.ambient_density = state.temperature_k();
    gc.use_bfecc = true;
    gc.smagorinsky_cs = config.smagorinsky_cs;
    gc.use_multigrid = true;
    gc
}

/// Create and initialize a pravash [`FluidGrid`] for atmospheric simulation.
///
/// The grid's density field is initialized with the ISA temperature profile
/// at the given altitude, which drives buoyancy-based convection when stepped
/// with the corresponding [`GridConfig`].
///
/// - `nx`, `ny`: grid dimensions (minimum 4x4)
/// - `state`: atmospheric state providing reference conditions
/// - `config`: flow simulation configuration
pub fn atmospheric_grid(
    nx: usize,
    ny: usize,
    state: &AtmosphericState,
    config: &AtmoFlowConfig,
) -> Result<FluidGrid> {
    let grid = FluidGrid::new(nx, ny, config.cell_size_m).map_err(|e| {
        BadalError::ComputationError(format!("failed to create atmospheric grid: {e}"))
    })?;
    // Initialize density field with temperature profile (temperature as scalar)
    let mut grid = grid;
    for y in 0..ny {
        let altitude = state.altitude_m() + (y as f64) * config.cell_size_m;
        let temp = atmosphere::standard_temperature(altitude);
        for x in 0..nx {
            let i = y * nx + x;
            grid.density[i] = temp;
        }
    }
    Ok(grid)
}

/// Apply Coriolis forcing to a pravash grid's velocity field.
///
/// In the Northern Hemisphere, Coriolis deflects flow to the right:
/// - du/dt = +f × v
/// - dv/dt = -f × u
///
/// - `grid`: fluid grid to modify
/// - `latitude_rad`: observer latitude (radians)
/// - `dt`: timestep (seconds)
pub fn apply_coriolis(grid: &mut FluidGrid, latitude_rad: f64, dt: f64) {
    let f = crate::wind::coriolis_parameter(latitude_rad);
    let n = grid.vx.len();
    for i in 0..n {
        let u = grid.vx[i];
        let v = grid.vy[i];
        grid.vx[i] += f * v * dt;
        grid.vy[i] -= f * u * dt;
    }
}

/// Apply pressure gradient forcing from a horizontal pressure field.
///
/// Computes -(1/ρ) × ∇P and adds it to the grid velocity.
///
/// - `grid`: fluid grid to modify
/// - `pressure_field`: pressure values per cell (Pa), row-major
/// - `density`: air density (kg/m³)
/// - `dt`: timestep (seconds)
pub fn apply_pressure_gradient(
    grid: &mut FluidGrid,
    pressure_field: &[f64],
    density: f64,
    dt: f64,
) -> Result<()> {
    let nx = grid.nx;
    let ny = grid.ny;
    if pressure_field.len() != nx * ny {
        return Err(BadalError::ComputationError(format!(
            "pressure field size {} != grid size {}",
            pressure_field.len(),
            nx * ny
        )));
    }
    if density <= 0.0 {
        return Err(BadalError::InvalidPressure(
            "density must be positive".into(),
        ));
    }
    let dx = grid.dx;
    let inv_rho_dx = 1.0 / (density * dx);

    for y in 1..ny - 1 {
        for x in 1..nx - 1 {
            let i = y * nx + x;
            let dp_dx = (pressure_field[i + 1] - pressure_field[i - 1]) * 0.5;
            let dp_dy = (pressure_field[i + nx] - pressure_field[i - nx]) * 0.5;
            grid.vx[i] -= dp_dx * inv_rho_dx * dt;
            grid.vy[i] -= dp_dy * inv_rho_dx * dt;
        }
    }
    Ok(())
}

/// Initialize a shallow water simulation for flood modeling from precipitation.
///
/// Creates a flat terrain shallow water grid and adds water from rainfall.
///
/// - `nx`, `ny`: grid dimensions
/// - `cell_size_m`: grid cell size in meters
/// - `rain_rate_mm_hr`: rainfall rate per cell (mm/hr), row-major
/// - `duration_hours`: duration of rainfall
pub fn flood_from_rainfall(
    nx: usize,
    ny: usize,
    cell_size_m: f64,
    rain_rate_mm_hr: &[f64],
    duration_hours: f64,
) -> Result<ShallowWater> {
    if rain_rate_mm_hr.len() != nx * ny {
        return Err(BadalError::ComputationError(format!(
            "rain field size {} != grid size {}",
            rain_rate_mm_hr.len(),
            nx * ny
        )));
    }
    let mut sw = ShallowWater::new(nx, ny, cell_size_m, 0.0).map_err(|e| {
        BadalError::ComputationError(format!("failed to create shallow water grid: {e}"))
    })?;
    // Convert mm/hr × hours → meters of water
    for (h, &rate) in sw.height.iter_mut().zip(rain_rate_mm_hr.iter()) {
        *h = rate.max(0.0) * duration_hours / 1000.0;
    }
    Ok(sw)
}

/// Add rainfall to an existing shallow water simulation.
///
/// Accumulates water from precipitation rate over a timestep.
///
/// - `sw`: shallow water state to modify
/// - `rain_rate_mm_hr`: rainfall rate per cell (mm/hr), row-major
/// - `dt_hours`: timestep duration (hours)
pub fn add_rainfall(sw: &mut ShallowWater, rain_rate_mm_hr: &[f64], dt_hours: f64) -> Result<()> {
    let n = sw.nx * sw.ny;
    if rain_rate_mm_hr.len() != n {
        return Err(BadalError::ComputationError(format!(
            "rain field size {} != grid size {}",
            rain_rate_mm_hr.len(),
            n
        )));
    }
    for (h, &rate) in sw.height.iter_mut().zip(rain_rate_mm_hr.iter()) {
        *h += rate.max(0.0) * dt_hours / 1000.0;
    }
    Ok(())
}

/// Extract wind field (u, v components in m/s) from a pravash grid.
///
/// Returns `(u_field, v_field)` as flat vectors, row-major.
#[must_use]
pub fn extract_wind_field(grid: &FluidGrid) -> (Vec<f64>, Vec<f64>) {
    (grid.vx.clone(), grid.vy.clone())
}

/// Compute wind speed and direction fields from a pravash grid.
///
/// Returns `(speed_ms, direction_deg)` as flat vectors, row-major.
/// Direction follows meteorological convention (degrees, wind FROM).
#[must_use]
pub fn extract_wind_speed_direction(grid: &FluidGrid) -> (Vec<f64>, Vec<f64>) {
    let n = grid.vx.len();
    let mut speed = Vec::with_capacity(n);
    let mut direction = Vec::with_capacity(n);
    for i in 0..n {
        speed.push(crate::wind::wind_speed(grid.vx[i], grid.vy[i]));
        direction.push(crate::wind::wind_direction(grid.vx[i], grid.vy[i]));
    }
    (speed, direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sea_level() -> AtmosphericState {
        AtmosphericState::sea_level()
    }

    // -- grid_config --

    #[test]
    fn grid_config_creates_valid() {
        let state = sea_level();
        let cfg = grid_config(&state, &AtmoFlowConfig::default());
        assert_eq!(cfg.dt, 1.0);
        assert!(cfg.use_multigrid);
        assert!(cfg.use_bfecc);
    }

    // -- atmospheric_grid --

    #[test]
    fn atmospheric_grid_creates_valid() {
        let state = sea_level();
        let grid = atmospheric_grid(16, 16, &state, &AtmoFlowConfig::default()).unwrap();
        assert_eq!(grid.nx, 16);
        assert_eq!(grid.ny, 16);
        // Bottom row should have near-surface temperature
        assert!((grid.density[0] - 288.15).abs() < 1.0);
    }

    #[test]
    fn atmospheric_grid_temperature_decreases_with_height() {
        let state = sea_level();
        let grid = atmospheric_grid(8, 8, &state, &AtmoFlowConfig::default()).unwrap();
        let bottom = grid.density[0]; // y=0
        let top = grid.density[7 * 8]; // y=7
        assert!(
            top < bottom,
            "temperature should decrease with height: bottom={bottom}, top={top}"
        );
    }

    #[test]
    fn atmospheric_grid_too_small_errors() {
        let state = sea_level();
        assert!(atmospheric_grid(2, 2, &state, &AtmoFlowConfig::default()).is_err());
    }

    // -- coriolis --

    #[test]
    fn coriolis_deflects_velocity() {
        let state = sea_level();
        let mut grid = atmospheric_grid(8, 8, &state, &AtmoFlowConfig::default()).unwrap();
        // Set uniform eastward wind
        grid.vx.fill(10.0);
        grid.vy.fill(0.0);
        let vy_before = grid.vy[0];
        apply_coriolis(&mut grid, 45.0_f64.to_radians(), 1.0);
        // Coriolis should deflect to the right (negative vy in NH for eastward flow)
        assert!(
            grid.vy[0] < vy_before,
            "Coriolis should deflect eastward wind southward in NH"
        );
    }

    // -- pressure gradient --

    #[test]
    fn pressure_gradient_accelerates_flow() {
        let state = sea_level();
        let mut grid = atmospheric_grid(8, 8, &state, &AtmoFlowConfig::default()).unwrap();
        let nx = grid.nx;
        let ny = grid.ny;
        // Create east-west pressure gradient (high west, low east)
        let mut pfield = vec![0.0; nx * ny];
        for y in 0..ny {
            for x in 0..nx {
                pfield[y * nx + x] = 101_325.0 - (x as f64) * 100.0;
            }
        }
        let vx_before = grid.vx[grid.nx / 2 + (grid.ny / 2) * grid.nx];
        apply_pressure_gradient(&mut grid, &pfield, 1.225, 1.0).unwrap();
        let vx_after = grid.vx[grid.nx / 2 + (grid.ny / 2) * grid.nx];
        assert!(
            vx_after > vx_before,
            "pressure gradient should accelerate flow from high to low pressure"
        );
    }

    #[test]
    fn pressure_gradient_wrong_size_errors() {
        let state = sea_level();
        let mut grid = atmospheric_grid(8, 8, &state, &AtmoFlowConfig::default()).unwrap();
        let pfield = vec![0.0; 10]; // wrong size
        assert!(apply_pressure_gradient(&mut grid, &pfield, 1.225, 1.0).is_err());
    }

    // -- shallow water / flood --

    #[test]
    fn flood_from_rainfall_creates_valid() {
        let rain = vec![10.0; 16]; // 10 mm/hr
        let sw = flood_from_rainfall(4, 4, 100.0, &rain, 2.0).unwrap();
        // 10 mm/hr × 2 hr = 20 mm = 0.02 m
        assert!((sw.height[0] - 0.02).abs() < 1e-6);
    }

    #[test]
    fn flood_wrong_size_errors() {
        let rain = vec![10.0; 10]; // wrong size for 4x4
        assert!(flood_from_rainfall(4, 4, 100.0, &rain, 1.0).is_err());
    }

    #[test]
    fn add_rainfall_accumulates() {
        let mut sw = ShallowWater::new(4, 4, 100.0, 0.0).unwrap();
        let rain = vec![5.0; 16]; // 5 mm/hr
        add_rainfall(&mut sw, &rain, 1.0).unwrap();
        // 5 mm/hr × 1 hr = 5 mm = 0.005 m
        assert!((sw.height[0] - 0.005).abs() < 1e-6);
        // Add more
        add_rainfall(&mut sw, &rain, 1.0).unwrap();
        assert!((sw.height[0] - 0.01).abs() < 1e-6);
    }

    // -- wind extraction --

    #[test]
    fn extract_wind_field_returns_correct_size() {
        let state = sea_level();
        let grid = atmospheric_grid(8, 8, &state, &AtmoFlowConfig::default()).unwrap();
        let (u, v) = extract_wind_field(&grid);
        assert_eq!(u.len(), 64);
        assert_eq!(v.len(), 64);
    }

    #[test]
    fn extract_wind_speed_direction_works() {
        let state = sea_level();
        let mut grid = atmospheric_grid(8, 8, &state, &AtmoFlowConfig::default()).unwrap();
        grid.vx.fill(5.0); // eastward
        grid.vy.fill(0.0);
        let (speed, dir) = extract_wind_speed_direction(&grid);
        assert!((speed[0] - 5.0).abs() < f64::EPSILON);
        // Eastward flow = wind FROM west = 270°
        assert!((dir[0] - 270.0).abs() < 1.0);
    }
}
