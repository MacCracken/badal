use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_standard_temperature(c: &mut Criterion) {
    c.bench_function("atmosphere/standard_temperature", |b| {
        b.iter(|| badal::atmosphere::standard_temperature(black_box(5000.0)));
    });
}

fn bench_standard_pressure(c: &mut Criterion) {
    c.bench_function("atmosphere/standard_pressure", |b| {
        b.iter(|| badal::atmosphere::standard_pressure(black_box(5000.0)));
    });
}

fn bench_air_density(c: &mut Criterion) {
    c.bench_function("atmosphere/air_density", |b| {
        b.iter(|| badal::atmosphere::air_density(black_box(101_325.0), black_box(288.15)));
    });
}

fn bench_saturation_vp(c: &mut Criterion) {
    c.bench_function("moisture/saturation_vapor_pressure", |b| {
        b.iter(|| badal::moisture::saturation_vapor_pressure(black_box(20.0)));
    });
}

fn bench_dew_point(c: &mut Criterion) {
    c.bench_function("moisture/dew_point", |b| {
        b.iter(|| badal::moisture::dew_point(black_box(20.0), black_box(50.0)));
    });
}

fn bench_heat_index(c: &mut Criterion) {
    c.bench_function("moisture/heat_index", |b| {
        b.iter(|| badal::moisture::heat_index(black_box(35.0), black_box(80.0)));
    });
}

fn bench_wet_bulb(c: &mut Criterion) {
    c.bench_function("moisture/wet_bulb_temperature", |b| {
        b.iter(|| badal::moisture::wet_bulb_temperature(black_box(30.0), black_box(50.0)));
    });
}

fn bench_coriolis(c: &mut Criterion) {
    c.bench_function("wind/coriolis_parameter", |b| {
        b.iter(|| badal::wind::coriolis_parameter(black_box(0.785)));
    });
}

fn bench_wind_chill(c: &mut Criterion) {
    c.bench_function("wind/wind_chill", |b| {
        b.iter(|| badal::wind::wind_chill(black_box(-10.0), black_box(30.0)));
    });
}

fn bench_cloud_base(c: &mut Criterion) {
    c.bench_function("cloud/cloud_base_altitude", |b| {
        b.iter(|| badal::cloud::cloud_base_altitude(black_box(25.0), black_box(15.0)));
    });
}

fn bench_beaufort(c: &mut Criterion) {
    c.bench_function("wind/beaufort_scale", |b| {
        b.iter(|| badal::wind::beaufort_scale(black_box(12.0)));
    });
}

fn bench_barometric_pressure(c: &mut Criterion) {
    c.bench_function("pressure/barometric_pressure", |b| {
        b.iter(|| {
            badal::pressure::barometric_pressure(
                black_box(5000.0),
                black_box(101_325.0),
                black_box(255.0),
            )
        });
    });
}

fn bench_classify_stability(c: &mut Criterion) {
    c.bench_function("stability/classify_stability", |b| {
        b.iter(|| badal::stability::classify_stability(black_box(0.008), black_box(true)));
    });
}

fn bench_cape_simple(c: &mut Criterion) {
    c.bench_function("stability/cape_simple", |b| {
        b.iter(|| {
            badal::stability::cape_simple(black_box(302.0), black_box(300.0), black_box(1000.0))
        });
    });
}

fn bench_rain_rate(c: &mut Criterion) {
    c.bench_function("precipitation/rain_rate", |b| {
        b.iter(|| {
            badal::precipitation::rain_rate(
                black_box(badal::cloud::CloudType::Cumulonimbus),
                black_box(2000.0),
            )
        });
    });
}

fn bench_precipitation_type(c: &mut Criterion) {
    c.bench_function("precipitation/precipitation_type", |b| {
        b.iter(|| {
            badal::precipitation::precipitation_type(
                black_box(badal::cloud::CloudType::Cumulonimbus),
                black_box(20.0),
                black_box(15.0),
                black_box(2000.0),
            )
        });
    });
}

fn bench_freezing_level(c: &mut Criterion) {
    c.bench_function("precipitation/freezing_level", |b| {
        b.iter(|| badal::precipitation::freezing_level(black_box(15.0)));
    });
}

fn bench_solar_zenith(c: &mut Criterion) {
    c.bench_function("radiation/solar_zenith_angle", |b| {
        b.iter(|| {
            badal::radiation::solar_zenith_angle(black_box(0.785), black_box(0.41), black_box(0.0))
        });
    });
}

fn bench_clear_sky_irradiance(c: &mut Criterion) {
    c.bench_function("radiation/clear_sky_irradiance", |b| {
        b.iter(|| badal::radiation::clear_sky_irradiance(black_box(0.5), black_box(0.75)));
    });
}

fn bench_longwave_emission(c: &mut Criterion) {
    c.bench_function("radiation/longwave_emission", |b| {
        b.iter(|| badal::radiation::longwave_emission(black_box(0.95), black_box(288.15)));
    });
}

fn bench_equilibrium_temp(c: &mut Criterion) {
    c.bench_function("radiation/equilibrium_temperature", |b| {
        b.iter(|| badal::radiation::equilibrium_temperature(black_box(1361.0), black_box(0.3)));
    });
}

fn bench_sea_land_breeze(c: &mut Criterion) {
    c.bench_function("mesoscale/sea_land_breeze", |b| {
        b.iter(|| {
            badal::mesoscale::sea_land_breeze(black_box(298.0), black_box(293.0), black_box(1000.0))
        });
    });
}

fn bench_katabatic_wind(c: &mut Criterion) {
    c.bench_function("mesoscale/katabatic_wind", |b| {
        b.iter(|| {
            badal::mesoscale::katabatic_wind(
                black_box(0.175),
                black_box(2000.0),
                black_box(5.0),
                black_box(280.0),
            )
        });
    });
}

fn bench_urban_heat_island(c: &mut Criterion) {
    c.bench_function("mesoscale/urban_heat_island", |b| {
        b.iter(|| {
            badal::mesoscale::urban_heat_island(
                black_box(1_000_000.0),
                black_box(3.0),
                black_box(0.3),
            )
        });
    });
}

fn bench_supercell_composite(c: &mut Criterion) {
    c.bench_function("severe/supercell_composite", |b| {
        b.iter(|| {
            badal::severe::supercell_composite(black_box(2000.0), black_box(25.0), black_box(200.0))
        });
    });
}

fn bench_significant_tornado(c: &mut Criterion) {
    c.bench_function("severe/significant_tornado", |b| {
        b.iter(|| {
            badal::severe::significant_tornado(
                black_box(3000.0),
                black_box(30.0),
                black_box(300.0),
                black_box(800.0),
            )
        });
    });
}

fn bench_derecho_composite(c: &mut Criterion) {
    c.bench_function("severe/derecho_composite", |b| {
        b.iter(|| {
            badal::severe::derecho_composite(black_box(2000.0), black_box(25.0), black_box(20.0))
        });
    });
}

criterion_group!(
    benches,
    bench_standard_temperature,
    bench_standard_pressure,
    bench_air_density,
    bench_saturation_vp,
    bench_dew_point,
    bench_heat_index,
    bench_wet_bulb,
    bench_coriolis,
    bench_wind_chill,
    bench_cloud_base,
    bench_beaufort,
    bench_barometric_pressure,
    bench_classify_stability,
    bench_cape_simple,
    bench_rain_rate,
    bench_precipitation_type,
    bench_freezing_level,
    bench_solar_zenith,
    bench_clear_sky_irradiance,
    bench_longwave_emission,
    bench_equilibrium_temp,
    bench_sea_land_breeze,
    bench_katabatic_wind,
    bench_urban_heat_island,
    bench_supercell_composite,
    bench_significant_tornado,
    bench_derecho_composite
);
criterion_main!(benches);
