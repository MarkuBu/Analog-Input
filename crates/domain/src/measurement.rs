use crate::analog_input::CalibrationState;
use crate::errors::DomainError;
use crate::raw::RawAdValue;
use crate::units::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementValue {
    Resistance(Ohm),
    Voltage(Volt),
    Current(MilliAmpere),
    Temperature(Celsius),
}

/// ⚠️ Thermoelement-Spannung ist KEIN normaler Messwert
#[derive(Debug, Clone, PartialEq)]
pub struct ThermocoupleVoltage(pub MilliVolt);

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    raw: RawAdValue,
    value: MeasurementValue,
}

impl Measurement {
    pub fn new(
        raw: RawAdValue,
        calibration: &CalibrationState,
        value: MeasurementValue,
    ) -> Result<Self, DomainError> {
        match calibration {
            CalibrationState::Calibrated { .. } => Ok(Self { raw, value }),
            CalibrationState::Uncalibrated => Err(DomainError::UncalibratedMeasurement),
        }
    }

    pub fn raw(&self) -> RawAdValue {
        self.raw
    }

    pub fn value(&self) -> &MeasurementValue {
        &self.value
    }
}
