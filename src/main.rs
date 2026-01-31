use domain::measurement_range::{MeasurementRange, Wiring};

fn main() {
    let r = MeasurementRange::Pt100 {
        wiring: Wiring::FourWire,
    };
    println!("{:?}", r);
}
