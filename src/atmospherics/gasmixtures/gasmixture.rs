use crate::atmospherics::gasmixtures::gastype::GasType;
use crate::constants::*;
use crate::errors::AtmosError;
use std::collections::HashMap;
use std::fmt;

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
        info!("New gas mixture created using from_empty");
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
        info!("New gas mixture created using new");
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
        info!("New gas mixture created using from_vecs; temperature {} and volume {}", temperature, volume);
        Ok(GasMixture {
            gases,
            temperature,
            volume
        })
    }

    /// Returns true if gas of type `gas_type` exists in the mixture and has >0 moles.. Pretty simple.
    /// ## Example
    /// ```rust
    ///
    /// ```
    pub fn gas_exists(&self, gas_type: &'a GasType) -> bool {
        self.gases.contains_key(gas_type) && self.get_moles(gas_type).unwrap() > 0.0 // get_moles is guaranteed by contains_key so this is safe
    }

    /// Returns true if gases list is completely empty.
    /// ## Example
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// let mut mix = GasMixture::from_empty(100.0, 100.0); // well, duh, now its empty
    ///
    /// assert_eq!(mix.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.gases.is_empty();
    }

    /// Guarantees that a `GasMixture` has a gas of type `gas_id`.
    /// Returns `true` if the gas already existed before calling `assert_gas()`.
    /// ## Example
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

    // TODO: Maybe figure out a more idiomatic way to do get_moles and change_moles

    /// Returns `Some(mole count)` of GasType if it exists, `None` otherwise.
    /// ## Example
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// use libatmos::constants::gases;
    /// // 50 mol O2, 50 mol N2, 100 K at 100 L
    /// let mut mix = GasMixture::from_vecs(vec![&gases::O2, &gases::N2], vec![50.0, 50.0], 100.0, 100.0)   ///
    ///
    /// assert_eq!(mix.get_moles(&gases::O2).unwrap(), 50.0);
    /// ```
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
        if !self.is_empty() { // P = nRT / V
            let pressure = (self.total_moles().unwrap() * num::R_IDEAL_GAS_EQUATION * self.temperature) / self.volume as f64;
            return Ok(pressure);
        }
        Err(AtmosError::GasMixtureEmpty)
    }

    /// Returns heat capacity of the mixture.
    ///
    /// ## Example
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// use libatmos::constants::gases;
    /// // 50 mol O2, 50 mol N2, 100 K at 100 L
    /// let mut mix = GasMixture::from_vecs(vec![&gases::O2, &gases::N2], vec![50.0, 50.0], 100.0, 100.0);
    /// assert_eq!(mix.heat_capacity().unwrap(), 2000.0) // mix heatcap = (50 * 20) * 2 = 2000
    /// ```
    pub fn heat_capacity(&self) -> Result<f64, AtmosError> {
        if !self.is_empty() {
            let sum = self.gases.iter()
                                .fold(0.0, |acc, (gastype, moles)| acc + ((gastype.specific_heat as f64) * moles)); // big boy
            return Ok(sum);
        }
        Err(AtmosError::GasMixtureEmpty)
    }

    /// Returns thermal energy of the mixture
    ///
    /// ## Example
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// use libatmos::constants::gases;
    /// // 50 mol O2, 50 mol N2, 100 K at 100 L
    /// let mut mix = GasMixture::from_vecs(vec![&gases::O2, &gases::N2], vec![50.0, 50.0], 100.0, 100.0);
    /// assert_eq!(mix.thermal_energy().unwrap(), 200_000.0) // mix thermal energy = 2000 (heatcap) * 100
    /// ```
    pub fn thermal_energy(&self) -> Result<f64, AtmosError> {
        if !self.is_empty() {
            let thermal_energy = self.heat_capacity().unwrap() * self.temperature;
            return Ok(thermal_energy);
        }
        Err(AtmosError::GasMixtureEmpty)
    }

    /// Merges two gas mixtures into self, adding together their gases mole counts and
    /// equalizing temperatures according to heat capacity.
    ///
    /// ## Example
    /// ```rust
    /// use libatmos::atmospherics::gasmixtures::gasmixture::GasMixture;
    /// ```
    pub fn merge(&mut self, giver: GasMixture<'a>) -> Result<(), AtmosError> {
        if self.is_empty() || giver.is_empty() {
            return Err(AtmosError::GasMixtureEmpty);
        }

        if (self.temperature - giver.temperature).abs() > num::MINIMUM_TEMP_DELTA_TO_CONSIDER {
            let self_heatcap = self.heat_capacity().unwrap();
            let giver_heatcap = giver.heat_capacity().unwrap();
            let combined_heatcap = giver_heatcap + self_heatcap;

            self.temperature = (giver.temperature * giver_heatcap + self.temperature * self_heatcap) / combined_heatcap;
        }

        for (gastype, moles) in giver.gases.iter() {
            self.assert_gas(gastype);
            self.change_moles(gastype, self.get_moles(gastype).unwrap() + moles).unwrap();
        }

        Ok(())
    }

    /// Removes a quantity of gas in `mol` from the gas mixture.
    ///
    /// ## Example
    pub fn remove(&mut self, mut amount: f64) -> Result<GasMixture, AtmosError> {
        let total_moles = self.total_moles().unwrap();
        if amount > total_moles {
            amount = total_moles;
        }
        if amount < 0.0 {
            return Err(AtmosError::LessThanZero { value: amount });
        }

        let mut removed = GasMixture::from_empty(self.temperature, self.volume);
        for (gastype, moles) in self.gases.clone().iter() { // clone is necessary to avoid mutability issues
            removed.assert_gas(gastype);
            removed.change_moles(gastype, (moles / total_moles) * amount).unwrap(); // percentage
            self.change_moles(gastype, moles - removed.get_moles(gastype).unwrap()).unwrap();
        }
        Ok(removed)
    }

    /// Removes a percentage / ratio of gas from the gas mixture, as opposed to `mol` count directly.
    ///
    /// ## Example
    pub fn remove_ratio(&mut self, mut ratio: f64) -> Result<GasMixture, AtmosError> {
        if ratio <= 0.0 {
            return Err(AtmosError::LessThanZero { value: ratio });
        }
        if ratio > 1.0 {
            ratio = 1.0;
        }

        let mut removed = GasMixture::from_empty(self.temperature, self.volume);
        for (gastype, moles) in self.gases.clone().iter() {
            removed.assert_gas(gastype);
            removed.change_moles(gastype, moles * ratio).unwrap();
            self.change_moles(gastype, moles - (moles * ratio)).unwrap();
        }
        Ok(removed)
    }

    // Not implementing these yet because they are rarely used except in environmental
    // atmos + machinery and I'm not coding that yet
    // TODO: Implement share() / temperature_share()

    /// It shares!
    /// ## Example
    pub fn share(&mut self, _sharer: GasMixture, _atmos_adjacent_turfs: f64) {
        unimplemented!();
    }

    /// It shares (temperature)!
    ///
    /// ## Example
    pub fn temperature_share(&self) {
        unimplemented!();
    }

    // TODO: Implement compare()?
    // TODO: Implement react()
}

impl<'a> fmt::Display for GasMixture<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut gas_string = String::new();
        for (gastype, moles) in self.gases.iter() {
            gas_string.push_str(format!("{}:{},", gastype.id, moles).as_str());
        }
        write!(f, "t:{},v:{},g:{}", self.temperature, self.volume, gas_string)
    }
}
