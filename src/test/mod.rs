use uom::si::Quantity;

const EPSILON: f32 = 1e-3;

/// Asserts that the expected quantity is equal to the actual value in base units.
pub fn assert_quantities_eq<D: uom::si::Dimension + ?Sized, U: uom::si::Units<f32> + ?Sized>(
    expected: Quantity<D, U, f32>,
    actual_in_base_units: f32,
) {
    assert!(
        (expected.value - actual_in_base_units).abs() < EPSILON,
        "Expected: {}, Actual: {}",
        expected.value,
        actual_in_base_units
    );
}
