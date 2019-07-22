pub const IDEAL_GAS: f32 = 8.31;
pub const ONE_ATM: f32 = 101.325;
pub const TEMP_MIN: f32 = 2.7;
pub const ZERO_CELS: f32 =  273.15;
pub const TANK_FRAG_PRESSURE: f32 = 40.0 * ONE_ATM;
pub const TANK_FRAG_SCALE: f32 = 6.0 * ONE_ATM;
pub const MIN_TEMP_DELTA: f32 = 0.5;
pub const MIN_HEAT_CAP: f32 = 0.0003;
pub const TANK_VOLUME: i32 = 70;
pub const CAN_VOLUME: i32 = 1000;

// Plasma fire constants

pub const OXY_BURN_RATE_BASE: f32 = 1.4;
pub const PLASMA_BURN_RATE_DELTA: i32 = 9;
pub const PLASMA_UPPER_TEMP: f32 = 1370.0 + ZERO_CELS;
pub const FIRE_MIN_TEMP: f32 = 100.0 + ZERO_CELS;
pub const PLASMA_OXY_FULLBURN: i32 = 10;
pub const CARBON_ENERGY_RELEASED: i32 = 100_000;
pub const HYDROGEN_ENERGY_RELEASED: i32 = 280_000;
pub const PLASMA_ENERGY_RELEASED: i32 = 3_000_000;

// Assmos

pub const NITRYL_FORM_ENERGY: i32 = 100_000;
pub const TRIT_BURN_OXY_FACTOR: i32 = 100;
pub const TRIT_BURN_TRIT_FACTOR: i32 = 10;
pub const SUPER_SATURATION_THRESHOLD: i32 = 96;

// Stim

pub const STIM_HEAT_SCALE: i32 = 100_000;
pub const STIM_FIRST_RISE: f32 = 0.65;
pub const STIM_FIRST_DROP: f32 = 0.065;
pub const STIM_SEC_RISE: f32 = 0.0009;
pub const STIM_ABS_DROP: f64 = 0.00000335;

// Other

pub const REACT_OPPRESSION_THRESHOLD: i32 = 5;