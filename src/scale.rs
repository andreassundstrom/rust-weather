pub fn scale_value(
    value: f32,
    min_value: f32,
    max_value: f32,
    min_range: i32,
    max_range: i32,
) -> f32 {
    if value > max_value || value < min_value {
        panic!("Value out of range")
    }
    let norm_value = (value - min_value) / (max_value - min_value);
    norm_value * max_range as f32 + min_range as f32
}

#[test]
fn scale_negativ_range_works() {
    let scaled_value = scale_value(0.0, -5.0, 5.0, 0, 1);
    assert_eq!(0.5, scaled_value);
}

#[test]
fn scale_positive_range_works() {
    let scaled_value = scale_value(5.0, 0.0, 10.0, 0, 100);
    assert_eq!(50.0, scaled_value);
}

#[test]
#[should_panic]
fn value_over_range_panics() {
    scale_value(50.0, 0.0, 49.0, 0, 10);
}

#[test]
#[should_panic]
fn value_under_range_panics() {
    scale_value(1.0, 2.0, 50.0, 0, 10);
}
