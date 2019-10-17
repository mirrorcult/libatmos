use crate::{
    constants::*,
    atmospherics::gases::*,
};

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
    // Mix contains just O2, at 69.42 mol
    let mut mix = GasMixture::from_vecs(vec![&gases::O2], vec![69.42], 273.15, 70).unwrap();
    assert_eq!(mix.gas_exists(&gases::N2), false); // Doesn't exist in mix  
    mix.assert_gas(&gases::N2);
    assert_eq!(mix.gas_exists(&gases::N2), false); // Even though we asserted, its still only at 0 mols so gas_exists fails
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

#[test]
fn merge() {
    let gas_vec = vec![&gases::BZ, &gases::MIASMA];
    let mole_vec = vec![50.0, 100.0];
    let giver = GasMixture::from_vecs(gas_vec, mole_vec, 100.0, 70).unwrap();
    let mut taker = giver.clone();

    taker.merge(giver).unwrap();
    assert_eq!(taker.get_moles(&gases::O2).unwrap(), 300.0); // added up
}
