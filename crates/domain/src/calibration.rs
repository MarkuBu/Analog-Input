use crate::raw::RawAdValue;
use crate::units::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalibrationConstants {
    offset: f64,
    slope: f64,
}

impl CalibrationConstants {
    pub fn new(offset: f64, slope: f64) -> Result<Self, CalibrationError> {
        if slope == 0.0 {
            return Err(CalibrationError::ZeroSlope);
        }

        Ok(Self { offset, slope })
    }

    pub fn offset(&self) -> f64 {
        self.offset
    }

    pub fn slope(&self) -> f64 {
        self.slope
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalibrationError {
    ZeroSlope,
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
