use crate::atmospherics::gasmixtures::gastype::GasType;
use crate::constants;
use crate::errors::AtmosError;
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
/// let mix = GasMixture::from_vecs(gas_vec, mole_vec, 273.15, 70).unwrap();
/// assert_eq!(mix.temperature, 273.15);
/// assert_eq!(mix.volume, 70);
/// ```
#[derive(PartialEq)]
pub struct GasMixture<'a> {
    /// HashMap of a reference to a static GasType (i.e. O2, plasma)
    /// and their mole count as an f64.
    gases: HashMap<&'a GasType, f64>,
    /// Temperature of the gas mixture.
    pub temperature: f64,
    /// Volume of the gas mixture.
    pub volume: usize
}

impl<'a> GasMixture<'a> {
    /// Creates an empty `GasMixture` given temperature and volume.
    /// A good default temperature would be `T20C`, or `293.15`, and a good
    /// default volume might be 70L or 1000L, the volumes of a tank and canister respectively.
    /// ## Example
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// let mix = GasMixture::from_empty(293.15, 1000);
    /// assert_eq!(mix.temperature, 293.15);
    /// ```
    pub fn from_empty(temperature: f64, volume: usize) -> GasMixture<'a> {
        let gases: HashMap<&'a GasType, f64> = HashMap::new();
        GasMixture {
            gases,
            temperature,
            volume
        }
    }
    /// Creates a new instance of a `GasMixture` normally. 
    /// `GasMixure::empty()` and `GasMixture::from_vecs` are preferred heavily.
    pub fn new(gases: HashMap<&'a GasType, f64>, temperature: f64, volume: usize) -> GasMixture {
        GasMixture { 
            gases,
            temperature,
            volume
        }
    }
    /// Creates a new instance of a `GasMixture` from two vectors.
    /// Errors if both vectors are different sizes.
    /// ## Example 
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// use libatmos::constants::gases;
    /// let gas_vec = vec![&gases::BZ, &gases::MIASMA];
    /// let mole_vec = vec![50.0, 500.5];
    /// let mix = GasMixture::from_vecs(gas_vec, mole_vec, 273.15, 70).unwrap();
    /// assert_eq!(mix.get_moles(&gases::BZ).unwrap(), 50.0);
    /// ```
    pub fn from_vecs(gas_types: Vec<&'a GasType>, moles: Vec<f64>, temperature: f64, volume: usize) -> Result<GasMixture, AtmosError> {
        if gas_types.len() != moles.len() { 
            return Err(AtmosError::VectorLengthMismatch { gas_length: gas_types.len(), mole_length: moles.len() });
        }
        
        // gas_ids contains a vec of references to static gastypes located in constants/gases.rs
        // this zips it into a hashmap with moles, so it looks like
        // [(constants::gases::O2, 5), (constants::gases::N2, 300.7)...]
        let gases = gas_types.into_iter()
                             .zip(moles.into_iter())
                             .collect::<HashMap<_, _>>();
        Ok(GasMixture {
            gases,
            temperature,
            volume
        })
    }
    /// Returns true if gas of type `gas_type` exists in the mixture and has >0 moles.. Pretty simple.
    pub fn gas_exists(&self, gas_type: &'a GasType) -> bool {
        self.gases.contains_key(gas_type) && self.get_moles(gas_type).unwrap() > 0.0 // get_moles is guaranteed by contains_key so this is safe 
    }

    /// Returns true if gases list is completely empty.
    pub fn is_empty(&self) -> bool {
        return self.gases.is_empty();
    }

    /// Guarantees that a `GasMixture` has a gas of type `gas_id`.
    /// Returns `true` if the gas already existed before calling `assert_gas()`.
    /// ## Usage
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// use libatmos::constants::gases;
    /// // Mix contains just O2, at 69.42 mol
    /// let mut mix = GasMixture::from_vecs(vec![&gases::O2], vec![69.42], 273.15, 70);
    /// assert_eq!(mix.gas_exists(&gases::N2), false); // Doesn't exist in mix yet
    /// 
    /// mix.assert_gas(&gases::N2); 
    /// assert_eq!(mix.gas_exists(&gases::N2), false); // Even though we asserted, its still only at 0 mols so gas_exists fails
    /// ```
    pub fn assert_gas(&mut self, gas_type: &'a GasType) -> bool {
        if self.gas_exists(gas_type) {
            return true
        } 
        self.gases.insert(gas_type, 0.0);
        false 
    }

    // add_gas would be here, but its functionally useless in this library
    // as its really only in SS13 for performance reasons

    /// Returns `Some(mole count)` of GasType if it exists, `None` otherwise.
    pub fn get_moles(&self, gas_type: &'a GasType) -> Option<f64> {
        if self.gas_exists(gas_type) {
            return Some(self.gases.get(gas_type)?.clone()); // not sure why get() returns a reference.. to a double.. when it could just clone but..
        }
        None
    }

    /// Changes the mole count of gas `gas_type` to `moles`. Errors if the gas isn't
    /// in the mixture.
    pub fn change_moles(&mut self, gas_type: &'a GasType, moles: f64) -> Result<(), AtmosError> {
        if self.gas_exists(gas_type) {
            self.gases.entry(gas_type)
                      .and_modify(|v| { *v = moles });
            return Ok(())
        }
        Err(AtmosError::GasNotFound { gas: gas_type })
    }

    /// Returns total mole count of the gas mixture.
    pub fn total_moles(&self) -> Result<f64, AtmosError> {
        if !self.is_empty() {
            let sum = self.gases.values()
                                .fold(0.0, |acc, x| acc + x);
            return Ok(sum)
        }
        Err(AtmosError::GasMixtureEmpty)
    }

    /// Returns pressure of the mixture in kPa.
    pub fn return_pressure(&self) -> Result<f64, AtmosError> {
        if !self.is_empty() {
            let pressure = (self.total_moles().unwrap() * constants::num::R_IDEAL_GAS_EQUATION * self.temperature) / self.volume as f64; 
            return Ok(pressure);
        }
        Err(AtmosError::GasMixtureEmpty)
    }

    /// Returns heat capacity of the mixture.
    pub fn heat_capacity(&self) -> Result<f64, AtmosError> {
        if !self.is_empty() {
            let sum = self.gases.iter()
                                .fold(0.0, |acc, (gastype, moles)| acc + ((gastype.specific_heat as f64) * moles));
            return Ok(sum);
        }
        Err(AtmosError::GasMixtureEmpty)
    }

    /// Returns thermal energy of the mixture
    pub fn thermal_energy(&self) -> Result<f64, AtmosError> {
        if !self.is_empty() {
            let thermal_energy = self.heat_capacity().unwrap() * self.temperature;
            return Ok(thermal_energy);
        }
        Err(AtmosError::GasMixtureEmpty)
    }
}