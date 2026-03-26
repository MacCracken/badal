use badal::atmosphere::AtmosphericState;
use badal::cloud;
use badal::pressure;
use badal::stability;
use badal::*;

#[test]
fn atmosphere_ideal_gas_consistency() {
    let s = AtmosphericState::at_altitude(5000.0);
    let p_check = s.density() * atmosphere::R_AIR * s.temperature_k();
    assert!(
        (s.pressure_pa() - p_check).abs() < 1.0,
        "ideal gas law should hold"
    );
}

#[test]
fn cloud_base_from_state() {
    let temp = 25.0;
    let dp = moisture::dew_point(temp, 50.0).unwrap();
    let base = cloud::cloud_base_altitude(temp, dp);
    assert!(
        base > 500.0 && base < 3000.0,
        "cloud base should be reasonable, got {base}"
    );
}

#[test]
fn stability_from_lapse() {
    let s = stability::classify_stability(0.012, false);
    assert_eq!(s, stability::StabilityClass::Unstable);
}

#[test]
fn beaufort_and_coriolis() {
    let f = wind::coriolis_parameter(45.0_f64.to_radians());
    assert!(f > 0.0);
    assert_eq!(wind::beaufort_scale(25.0), 10); // Storm
}

#[test]
fn end_to_end_weather_profile() {
    // Build a complete weather picture at 3000m
    let state = AtmosphericState::at_altitude(3000.0);
    assert!(state.temperature_k() > 200.0 && state.temperature_k() < 300.0);

    let es = moisture::saturation_vapor_pressure(state.temperature_celsius());
    assert!(es > 0.0);

    let dp = moisture::dew_point(state.temperature_celsius(), 60.0).unwrap();
    let base = cloud::cloud_base_altitude(state.temperature_celsius(), dp);
    assert!(base > 0.0);

    let f = wind::coriolis_parameter(45.0_f64.to_radians());
    let vg = pressure::geostrophic_wind_speed(0.01, state.density(), f);
    assert!(vg > 0.0);
}

#[test]
fn validated_constructor_rejects_invalid() {
    assert!(AtmosphericState::new(0.0, 101_325.0, 50.0, 0.0).is_err());
    assert!(AtmosphericState::new(-10.0, 101_325.0, 50.0, 0.0).is_err());
    assert!(AtmosphericState::new(288.15, -1.0, 50.0, 0.0).is_err());
    assert!(AtmosphericState::new(288.15, 101_325.0, -1.0, 0.0).is_err());
    assert!(AtmosphericState::new(288.15, 101_325.0, 101.0, 0.0).is_err());
}

#[test]
fn validated_constructor_accepts_valid() {
    let s = AtmosphericState::new(288.15, 101_325.0, 50.0, 0.0).unwrap();
    assert_eq!(s, AtmosphericState::sea_level());
}
