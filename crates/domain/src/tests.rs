use crate::measurement_range::{MeasurementRange, ThermocoupleType};

#[test]
fn thermocouple_requires_type() {
    let tc = MeasurementRange::Thermocouple {
        tc_type: ThermocoupleType::K,
    };

    // Wenn das kompiliert, ist der Test bestanden.
    // Ein Thermoelement ohne Typ kann nicht existieren.
    assert!(matches!(tc, MeasurementRange::Thermocouple { .. }));
}

use crate::analog_input::CalibrationState;
use crate::calibration::CalibrationConstants;

#[test]
fn calibrated_state_requires_constants() {
    let constants = CalibrationConstants {
        offset: 0.0,
        slope: 1.0,
    };

    let state = CalibrationState::Calibrated { constants };

    match state {
        CalibrationState::Calibrated { constants } => {
            assert_eq!(constants.slope, 1.0);
        }
        _ => panic!("Expected calibrated state"),
    }
}

use crate::measurement_range::{MeasurementRange, Wiring};

#[test]
fn measurement_ranges_are_closed() {
    let r1 = MeasurementRange::Pt100 {
        wiring: Wiring::FourWire,
    };

    let r2 = MeasurementRange::Voltage0To10V;

    match r1 {
        MeasurementRange::Pt100 {
            wiring: Wiring::FourWire,
        } => {}
        _ => panic!("Unexpected measurement range"),
    }

    match r2 {
        MeasurementRange::Voltage0To10V => {}
        _ => panic!("Unexpected measurement range"),
    }
}

use std::collections::HashMap;

use crate::analog_input::{AnalogInput, CalibrationState, Channel};
use crate::measurement_range::{MeasurementRange, Wiring};

#[test]
fn analog_input_has_explicit_measurement_ranges() {
    let mut ranges = HashMap::new();

    ranges.insert(
        MeasurementRange::Pt100 {
            wiring: Wiring::FourWire,
        },
        CalibrationState::Uncalibrated,
    );

    let input = AnalogInput {
        channel: Channel(1),
        ranges,
    };

    assert_eq!(input.channel.0, 1);
    assert_eq!(input.ranges.len(), 1);
}
