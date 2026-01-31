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
    channel: Channel,
    ranges: HashMap<MeasurementRange, CalibrationState>,
}

impl AnalogInput {
    pub fn new(
        channel: Channel,
        ranges: HashMap<MeasurementRange, CalibrationState>,
    ) -> Result<Self, AnalogInputError> {
        if ranges.is_empty() {
            return Err(AnalogInputError::NoMeasurementRanges);
        }

        Ok(Self { channel, ranges })
    }

    pub fn channel(&self) -> Channel {
        self.channel
    }

    pub fn ranges(&self) -> &HashMap<MeasurementRange, CalibrationState> {
        &self.ranges
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnalogInputError {
    NoMeasurementRanges,
}
