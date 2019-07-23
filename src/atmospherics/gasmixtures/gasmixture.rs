use std::collections::HashMap;

/// GasMixture struct. "Unit" type for pretty much everything
/// atmospheric in the game. Holds a list of gases, their amounts
/// (in mols) and the temperature of the mixture for various calculations.
/// ## Usage
/// ```rust
/// let
/// let gm = GasMixture::new()
pub struct GasMixture<'a> {
    gases: HashMap<&'a str, f64>,
    temperature: f64,
    volume: usize
}

impl<'a> GasMixture<'a> {
    /// Creates a new instance of a GasMixture normally.
    pub fn new(gases: HashMap<&'a str, f64>, temperature: f64, volume: usize) -> GasMixture {
        GasMixture { 
            gases,
            temperature,
            volume
        }
    }
    /// Creates a new instance of a GasMixture from two vectors.
    pub fn from_vecs(gas_ids: Vec<&'a str>, moles: Vec<f64>, temperature: f64, volume: usize) -> GasMixture {
        if gas_ids.len() != moles.len() { 
            panic!() // TODO actual error handling
        }
        
        // gas_ids contains a vec of ids that correspond to gases--"o2", "n2", etc
        // this zips it into a hashmap with moles, so it looks like
        // [("o2", 5), ("n2", 300.7)...]
        let gases = gas_ids.into_iter().zip(moles.into_iter()).collect::<HashMap<_, _>>();
        GasMixture {
            gases,
            temperature,
            volume
        }
    }
    /// Guarantees that a GasMixture has a gas of type gas_id
    pub fn assert_gas(&mut self, gas_id: &'a str) {
       self.gases.insert(gas_id, 0.0);
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