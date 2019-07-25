use crate::atmospherics::gasmixtures::gastype::GasType;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AtmosError<'a> {
    #[snafu(display("Couldn't find gas '{}'. Did you check that it exists or call assert_gas() beforehand?", gas))]
    GasNotFound { gas: &'a GasType }
}