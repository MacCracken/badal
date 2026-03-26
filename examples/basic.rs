use badal::{atmosphere, cloud, moisture, stability, wind};

fn main() {
    let state = atmosphere::AtmosphericState::at_altitude(2000.0);
    println!(
        "At 2000m: T={:.1}K ({:.1}°C), P={:.0}Pa, ρ={:.3}kg/m³",
        state.temperature_k(),
        state.temperature_celsius(),
        state.pressure_pa(),
        state.density()
    );

    let es = moisture::saturation_vapor_pressure(20.0);
    println!("Saturation VP at 20°C: {es:.0} Pa");

    let dp = moisture::dew_point(25.0, 60.0).expect("valid humidity");
    let base = cloud::cloud_base_altitude(25.0, dp);
    println!("Dew point at 25°C/60%: {dp:.1}°C → cloud base: {base:.0}m");

    let f = wind::coriolis_parameter(45.0_f64.to_radians());
    println!("Coriolis at 45°N: {f:.6} s⁻¹");

    let wc = wind::wind_chill(-10.0, 30.0);
    println!("Wind chill at -10°C/30km/h: {wc:.1}°C");

    let b = wind::beaufort_scale(15.0);
    println!("15 m/s → Beaufort {b}");

    let s = stability::classify_stability(0.008, true);
    println!("Lapse 8°C/km saturated → {s:?}");
}
