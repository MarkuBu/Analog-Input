#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Wiring {
    TwoWire,
    ThreeWire,
    FourWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThermocoupleType {
    K,
    J,
    T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MeasurementRange {
    Pt100 { wiring: Wiring },
    Pt1000 { wiring: Wiring },
    Thermocouple { tc_type: ThermocoupleType },
    Voltage0To1V,
    Voltage0To10V,
    Current0To20mA,
}
