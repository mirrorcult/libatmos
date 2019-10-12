pub use self::{
    gasmixture::GasMixture,
    gastype::GasType,
};

/// gas type struct
pub mod gastype;
/// gas mixture struct and functions
pub mod gasmixture;
/// reaction functions and react loop code
pub mod reactions;
