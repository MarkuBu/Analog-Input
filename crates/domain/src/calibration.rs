use crate::raw::RawAdValue;
use crate::units::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalibrationConstants {
    pub offset: f64,
    pub slope: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationPoint {
    pub reference: ReferenceValue,
    pub raw: RawAdValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReferenceValue {
    Resistance(Ohm),
    Voltage(Volt),
    ThermocoupleVoltage(MilliVolt),
}
