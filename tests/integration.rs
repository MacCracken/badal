use badal::*;
use badal::atmosphere::AtmosphericState;
use badal::cloud;
use badal::stability;
use badal::pressure;

#[test]
fn atmosphere_ideal_gas_consistency() {
    let s = AtmosphericState::at_altitude(5000.0);
    let p_check = s.density() * atmosphere::R_AIR * s.temperature_k;
    assert!((s.pressure_pa - p_check).abs() < 1.0, "ideal gas law should hold");
}

#[test]
fn cloud_base_from_state() {
    let temp = 25.0;
    let dp = badal::atmosphere::dew_point(temp, 50.0);
    let base = cloud::cloud_base_altitude(temp, dp);
    assert!(base > 500.0 && base < 3000.0, "cloud base should be reasonable, got {base}");
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
