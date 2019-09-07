use crate::*;
use constants::*;
use atmospherics::*;
use gasmixtures::*;
use gasmixture::GasMixture;

#[test]
fn from_vecs() {
    let gas_vec = vec![&gases::BZ, &gases::MIASMA];
    let mole_vec = vec![50.0, 100.0];
    let mix = GasMixture::from_vecs(gas_vec, mole_vec, 100.0, 70).unwrap();
    assert_eq!(mix.temperature, 100.0);
    assert_eq!(mix.volume, 70);
}

#[test]
fn gas_exists() {

}

#[test]
fn is_empty() {
    let mix = GasMixture::from_empty(100.0, 100); // well, duh, now its empty

    assert_eq!(mix.is_empty(), true);
}

#[test]
fn assert_gas() {

}

#[test]
fn get_moles() {

}

#[test]
fn change_moles() {

}

#[test]
fn total_moles() {

}

#[test]
fn return_pressure() {

}

#[test]
fn heat_capacity() {
    let gas_vec = vec![&gases::BZ, &gases::MIASMA];
    let mole_vec = vec![50.0, 100.0];
    let mix = GasMixture::from_vecs(gas_vec, mole_vec, 100.0, 70).unwrap();

    assert_eq!(mix.heat_capacity().unwrap(), 3000.0); // (50 * 20) + (100 * 20) = 3000
}

#[test]
fn thermal_energy() {
    let gas_vec = vec![&gases::BZ, &gases::MIASMA];
    let mole_vec = vec![50.0, 100.0];
    let mix = GasMixture::from_vecs(gas_vec, mole_vec, 100.0, 70).unwrap();

    assert_eq!(mix.thermal_energy().unwrap(), 300_000.0)
}
