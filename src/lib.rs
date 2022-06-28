/// Performs an addition
pub fn add(addend_a: f64, addend_b: f64) -> f64 {
    addend_a + addend_b
}

/// Performs a subtraction
pub fn sub(minuend: f64, substrahend: f64) -> f64 {
    minuend - substrahend
}

/// Performs a multiplication
pub fn mul(multiplicand: f64, multiplier: f64) -> f64 {
    multiplicand * multiplier
}

/// Performs a division
pub fn div(divisor: f64, dividend: f64) -> Result<f64, &'static str> {
    if dividend == 0.0 {
        return Err("Dividend is 0.");
    }

    Ok(divisor / dividend)
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
        assert_eq!(result.unwrap(), 1.0);

        let result = super::div(-10.0, 2.0);
        assert_eq!(result.unwrap(), -5.0);

        let result = super::div(2.0, 10.0);
        assert_eq!(result.unwrap(), 0.2);

        let result = super::div(2.0, -10.0);
        assert_eq!(result.unwrap(), -0.2);

        let result = super::div(0.0, 2.0);
        assert_eq!(result.unwrap(), 0.0);

        let result = super::div(2.0, 0.0);
        assert!(result.is_err());
    }
}
