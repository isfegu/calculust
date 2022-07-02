/// Performs an addition
#[no_mangle]
pub extern "C" fn add(addend_a: f64, addend_b: f64) -> f64 {
    addend_a + addend_b
}

/// Performs a subtraction
#[no_mangle]
pub extern "C" fn sub(minuend: f64, substrahend: f64) -> f64 {
    minuend - substrahend
}

/// Performs a multiplication
#[no_mangle]
pub extern "C" fn mul(multiplicand: f64, multiplier: f64) -> f64 {
    multiplicand * multiplier
}

/// Performs a division
#[no_mangle]
pub extern "C" fn div(divisor: f64, dividend: f64) -> f64 {
    if dividend == 0.0 {
        return 0.0;
    }

    divisor / dividend
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_works() {
        let result = super::add(2.0, 2.0);
        assert_eq!(result, 4.0);

        let result = super::add(-8.0, 2.0);
        assert_eq!(result, -6.0);
    }

    #[test]
    fn sub_works() {
        let result = super::sub(2.0, 2.0);
        assert_eq!(result, 0.0);

        let result = super::sub(2.0, 10.0);
        assert_eq!(result, -8.0);
    }

    #[test]
    fn mul_works() {
        let result = super::mul(2.0, 2.0);
        assert_eq!(result, 4.0);

        let result = super::mul(-2.0, 10.0);
        assert_eq!(result, -20.0);
    }

    #[test]
    fn div_works() {
        let result = super::div(2.0, 2.0);
        assert_eq!(result, 1.0);

        let result = super::div(-10.0, 2.0);
        assert_eq!(result, -5.0);

        let result = super::div(2.0, 10.0);
        assert_eq!(result, 0.2);

        let result = super::div(2.0, -10.0);
        assert_eq!(result, -0.2);

        let result = super::div(0.0, 2.0);
        assert_eq!(result, 0.0);

        let result = super::div(2.0, 0.0);
        assert_eq!(result, 0.0);
    }
}
