use std::collections::HashMap;

use crate::analog_input::{AnalogInput, CalibrationState, Channel};
use crate::calibration::CalibrationConstants;
use crate::measurement::Measurement;
use crate::measurement::MeasurementValue;
use crate::measurement_range::{MeasurementRange, ThermocoupleType, Wiring};
use crate::raw::RawAdValue;
use crate::units::Ohm;

//
// ─────────────────────────────────────────────────────────────
// Typ-Sicherheits-Tests
// ─────────────────────────────────────────────────────────────
//

#[test]
fn thermocouple_requires_type() {
    let tc = MeasurementRange::Thermocouple {
        tc_type: ThermocoupleType::K,
    };

    assert!(matches!(tc, MeasurementRange::Thermocouple { .. }));
}

#[test]
fn measurement_ranges_are_closed() {
    let r = MeasurementRange::Pt100 {
        wiring: Wiring::FourWire,
    };

    match r {
        MeasurementRange::Pt100 {
            wiring: Wiring::FourWire,
        } => {}
        _ => panic!("unexpected range"),
    }
}

//
// ─────────────────────────────────────────────────────────────
// Kalibrierkonstanten
// ─────────────────────────────────────────────────────────────
//

#[test]
fn calibration_constants_require_non_zero_slope() {
    let result = CalibrationConstants::new(0.0, 0.0);
    assert!(result.is_err());
}

#[test]
fn calibration_constants_can_be_created() {
    let c = CalibrationConstants::new(0.1, 1.0).unwrap();
    assert_eq!(c.slope(), 1.0);
}

//
// ─────────────────────────────────────────────────────────────
// Analoger Eingang
// ─────────────────────────────────────────────────────────────
//

#[test]
fn analog_input_requires_measurement_range() {
    let ranges = HashMap::new();
    let result = AnalogInput::new(Channel(1), ranges);
    assert!(result.is_err());
}

#[test]
fn analog_input_with_range_is_valid() {
    let mut ranges = HashMap::new();

    ranges.insert(
        MeasurementRange::Pt100 {
            wiring: Wiring::FourWire,
        },
        CalibrationState::Uncalibrated,
    );

    let input = AnalogInput::new(Channel(1), ranges).unwrap();
    assert_eq!(input.channel().0, 1);
}

//
// ─────────────────────────────────────────────────────────────
// Messung
// ─────────────────────────────────────────────────────────────
//

#[test]
fn measurement_requires_calibrated_state() {
    let raw = RawAdValue(42);

    let result = Measurement::new(
        raw,
        &CalibrationState::Uncalibrated,
        MeasurementValue::Resistance(Ohm(100.0)),
    );

    assert!(result.is_err());
}

#[test]
fn measurement_with_calibration_is_allowed() {
    let raw = RawAdValue(42);

    let constants = CalibrationConstants::new(0.0, 1.0).unwrap();

    let state = CalibrationState::Calibrated { constants };

    let measurement =
        Measurement::new(raw, &state, MeasurementValue::Resistance(Ohm(100.0))).unwrap();

    match measurement.value() {
        MeasurementValue::Resistance(r) => assert_eq!(r.0, 100.0),
        _ => panic!("unexpected measurement value"),
    }
}
