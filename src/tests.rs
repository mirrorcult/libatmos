use crate::*;
use constants::*;
use atmospherics::*;
use gasmixtures::*;
use machinery::*;
use environmental::*;

#[test]
fn create_gas_mixture() {
    let gas_vec = vec!["o2", "n2"];
    let mole_vec = vec![50.0, 500.5];
    let mix = gasmixture::GasMixture::from_vecs(gas_vec, mole_vec, 273.15, 70);
    assert_eq!(mix.get_temperature(), 273.15);
    assert_eq!(mix.volume, 70);
}