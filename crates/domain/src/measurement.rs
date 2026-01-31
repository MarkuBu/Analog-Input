use crate::raw::RawAdValue;
use crate::units::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementValue {
    Resistance(Ohm),
    Voltage(Volt),
    Current(MilliAmpere),
    Temperature(Celsius),
    ThermocoupleVoltage(MilliVolt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    pub raw: RawAdValue,
    pub value: MeasurementValue,
}
