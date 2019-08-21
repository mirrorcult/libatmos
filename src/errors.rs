use crate::atmospherics::gasmixtures::gastype::GasType;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AtmosError<'a> {
    #[snafu(display("Vectors in from_vecs were not the same length! \n Length of gas_vec: {} \n Length of mole_vec: {}", gas_length, mole_length))]
    VectorLengthMismatch { gas_length: usize, mole_length: usize }, 
    #[snafu(display("Couldn't find gas '{:?}'. Did you check that it exists or call assert_gas() beforehand?", gas))]
    GasNotFound { gas: &'a GasType },
    #[snafu(display("Gas mixture was empty! Did you forget to call assert_gas() or add any moles beforehand?"))]
    GasMixtureEmpty
}