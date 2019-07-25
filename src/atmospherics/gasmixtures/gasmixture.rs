use crate::atmospherics::gasmixtures::gastype::GasType;
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
    /// let mix = GasMixture::empty(293.15, 1000);
    /// assert_eq(mix.temperature) 
    pub fn empty(temperature: f64, volume: usize) -> GasMixture<'a> {
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
    /// ## Example 
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// use libatmos::constants::gases;
    /// let gas_vec = vec![&gases::BZ, &gases::MIASMA];
    /// let mole_vec = vec![50.0, 500.5];
    /// let mix = GasMixture::from_vecs(gas_vec, mole_vec, 273.15, 70);
    /// assert_eq(mix.get_moles(&gases::BZ), 50.0);
    /// ```
    pub fn from_vecs(gas_types: Vec<&'a GasType>, moles: Vec<f64>, temperature: f64, volume: usize) -> GasMixture {
        if gas_types.len() != moles.len() { 
            panic!() // TODO: actual error handling
        }
        
        // gas_ids contains a vec of references to static gastypes located in constants/gases.rs
        // this zips it into a hashmap with moles, so it looks like
        // [(constants::gases::O2, 5), (constants::gases::N2, 300.7)...]
        let gases = gas_types.into_iter().zip(moles.into_iter()).collect::<HashMap<_, _>>();
        GasMixture {
            gases,
            temperature,
            volume
        }
    }
    /// Returns true if gas of type `gas_type` exists in the mixture and has >0 moles.. Pretty simple.
    pub fn gas_exists(&self, gas_type: &'a GasType) -> bool {
        self.gases.contains_key(gas_type) && self.get_moles(gas_type).unwrap() > 0.0 // get_moles is guaranteed by contains_key so this is safe 
    }

    /// Guarantees that a `GasMixture` has a gas of type `gas_id`
    /// Returns `true` if the gas already existed before calling `assert_gas()`
    /// ## Usage
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// use libatmos::constants::gases;
    /// // Mix contains just O2, at 69.42 mol
    /// let mix = GasMixture::from_vecs(vec![&gases::O2], vec![69.42], 273.15, 70);
    /// assert_eq(mix.gas_exists(&gases::N2), false); // Doesn't exist in mix yet
    /// 
    /// mix.assert_gas(&gases::N2); 
    /// assert_eq(mix.gas_exists(&gases::N2), true); // Exists now
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
            return Some(self.gases.get(gas_type)?.clone()); // not sure why get() returns a reference when it could just implicit copy but..
        }
        None
    }
    /// Changes the mole count of gas `gas_type` to `moles`. Errors if the gas isn't
    /// in the mixture.
    pub fn change_moles(&mut self, gas_type: &'a GasType, moles: f64) -> Result<(), AtmosError> {
        if self.gas_exists(gas_type) {
            self.gases.entry(gas_type).and_modify(|v| { *v = moles });
            return Ok(())
        }
        Err(AtmosError::GasNotFound { gas: gas_type })
    }
}