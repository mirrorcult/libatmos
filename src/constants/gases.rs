use crate::atmospherics::gasmixtures::gastype::GasType;

pub static O2: GasType = GasType {
    id: "o2",
    name: "Oxygen",
    specific_heat: 20,
    fusion_power: 0
};

pub static N2: GasType = GasType {
    id: "n2",
    name: "Nitrogen",
    specific_heat: 20,
    fusion_power: 0
};

pub static CO2: GasType = GasType {
    id: "co2",
    name: "Carbon Dioxide",
    specific_heat: 30,
    fusion_power: 0
};

pub static PLASMA: GasType = GasType {
    id: "plasma",
    name: "Plasma",
    specific_heat: 200,
    fusion_power: 0
};

pub static H2O: GasType = GasType {
    id: "water_vapor",
    name: "Water Vapor",
    specific_heat: 40,
    fusion_power: 8
};

pub static HYPERNOB: GasType = GasType {
    id: "nob",
    name: "Hyper-noblium",
    specific_heat: 2000,
    fusion_power: 0
};

pub static N2O: GasType = GasType {
    id: "n2o",
    name: "Nitrous Oxide",
    specific_heat: 40,
    fusion_power: 10
};

pub static NO2: GasType = GasType {
    id: "no2",
    name: "Nitryl",
    specific_heat: 20,
    fusion_power: 16
};

pub static TRIT: GasType = GasType {
    id: "tritium",
    name: "Tritium",
    specific_heat: 10,
    fusion_power: 1
};

pub static BZ: GasType = GasType {
    id: "bz",
    name: "BZ",
    specific_heat: 20,
    fusion_power: 8
};

pub static STIM: GasType = GasType {
    id: "stim",
    name: "Stimulum",
    specific_heat: 5,
    fusion_power: 7
};

pub static PLUOX: GasType = GasType {
    id: "pluox",
    name: "Pluoxium",
    specific_heat: 80,
    fusion_power: -10
};

pub static MIASMA: GasType = GasType {
    id: "miasma",
    name: "Miasma",
    specific_heat: 20,
    fusion_power: 0
};
