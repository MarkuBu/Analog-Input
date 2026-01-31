use std::collections::HashMap;

use crate::calibration::CalibrationConstants;
use crate::measurement_range::MeasurementRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Channel(pub u8);

#[derive(Debug, Clone, PartialEq)]
pub enum CalibrationState {
    Uncalibrated,
    Calibrated { constants: CalibrationConstants },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalogInput {
    pub channel: Channel,
    pub ranges: HashMap<MeasurementRange, CalibrationState>,
}
