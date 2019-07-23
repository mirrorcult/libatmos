use crate::atmospherics::gasmixtures::gastype::GasType;
use std::collections::HashMap;

/// GasMixture struct. "Unit" type for pretty much everything
/// atmospheric in the game. Holds a list of gases, their amounts
/// (in mols) and the temperature of the mixture for various calculations.
/// ## Example 
/// ```rust
/// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
/// use libatmos::constants::gases;
/// let gas_vec = vec![&gases::BZ, &gases::MIASMA];
/// let mole_vec = vec![50.0, 500.5];
/// let mix = GasMixture::from_vecs(gas_vec, mole_vec, 273.15, 70);
/// assert_eq!(mix.get_temperature(), 273.15);
/// assert_eq!(mix.get_volume(), 70);
/// ```
#[derive(PartialEq)]
pub struct GasMixture<'a> {
    /// HashMap of a reference to a static GasType (i.e. O2, plasma)
    /// and their mole count as an f64.
    gases: HashMap<&'a GasType, f64>,
    /// Temperature of the gas mixture.
    temperature: f64,
    /// Volume of the gas mixture.
    volume: usize
}

impl<'a> GasMixture<'a> {
    /// Creates a new instance of a GasMixture normally.
    pub fn new(gases: HashMap<&'a GasType, f64>, temperature: f64, volume: usize) -> GasMixture {
        GasMixture { 
            gases,
            temperature,
            volume
        }
    }
    /// Creates a new instance of a GasMixture from two vectors.
    pub fn from_vecs(gas_types: Vec<&'a GasType>, moles: Vec<f64>, temperature: f64, volume: usize) -> GasMixture {
        if gas_types.len() != moles.len() { 
            panic!() // TODO actual error handling
        }
        
        // gas_ids contains a vec of ids that correspond to gases--"o2", "n2", etc
        // this zips it into a hashmap with moles, so it looks like
        // [("o2", 5), ("n2", 300.7)...]
        let gases = gas_types.into_iter().zip(moles.into_iter()).collect::<HashMap<_, _>>();
        GasMixture {
            gases,
            temperature,
            volume
        }
    }
    /// Guarantees that a GasMixture has a gas of type gas_id
    pub fn assert_gas(&mut self, gas_type: &'a GasType) {
       self.gases.insert(gas_type, 0.0);
    }
    /// Gets temperature of GasMixture
    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }
    /// Gets volume of GasMixture
    pub fn get_volume(&self) -> usize {
        self.volume
    }
}