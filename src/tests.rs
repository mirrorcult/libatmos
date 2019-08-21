use crate::*;
use constants::*;
use atmospherics::*;
use gasmixtures::*;

#[test]
fn create_gas_mixture() {
    let gas_vec = vec![&gases::BZ, &gases::MIASMA];
    let mole_vec = vec![50.0, 100.0];
    let mix = gasmixture::GasMixture::from_vecs(gas_vec, mole_vec, 100.0, 70).unwrap();
    assert_eq!(mix.temperature, 100.0);
    assert_eq!(mix.volume, 70);
}

#[test]
fn heat_capacity() {
    let gas_vec = vec![&gases::BZ, &gases::MIASMA];
    let mole_vec = vec![50.0, 100.0];
    let mix = gasmixture::GasMixture::from_vecs(gas_vec, mole_vec, 100.0, 70).unwrap();

    assert_eq!(mix.heat_capacity().unwrap(), 3000.0); // (50 * 20) + (100 * 20) = 3000
}

#[test]
fn thermal_energy() {
    let gas_vec = vec![&gases::BZ, &gases::MIASMA];
    let mole_vec = vec![50.0, 100.0];
    let mix = gasmixture::GasMixture::from_vecs(gas_vec, mole_vec, 100.0, 70).unwrap();   

    assert_eq!(mix.thermal_energy().unwrap(), 300_000.0)
}