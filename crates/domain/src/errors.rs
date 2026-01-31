#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    InvalidConfiguration,
    UncalibratedMeasurement,
    UnsupportedMeasurementRange,
}
