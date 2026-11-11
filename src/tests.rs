use crate::{Answer, Parser};

#[cfg(test)]
mod eq_tests {
    use crate::tests::{assert_close, test_bool};

    #[test]
    fn mul_over_add() {
        assert_close("2 + 3 * 4", 14.0);
    }
    #[test]
    fn sub_left_assoc() {
        assert_close("10 - 4 - 3", 3.0);
    }
    #[test]
    fn pow_over_mul() {
        assert_close("2 * 3 ^ 2", 18.0);
    }
    #[test]
    fn pow_right_assoc() {
        assert_close("2 ^ 3 ^ 2", 512.0);
    }
    #[test]
    fn neg_stops_at_add() {
        assert_close("-2 + 3", 1.0);
    }
    #[test]
    fn pow_over_neg() {
        assert_close("-2 ^ 2", -4.0);
    }
    #[test]
    fn bang_over_pow() {
        assert_close("2 ^ 3!", 64.0);
    }
    #[test]
    fn sqrt_stops_at_add() {
        assert_close("√4 + 5", 7.0);
    }
    #[test]
    fn trig_arg_is_full_expr() {
        assert_close("cos(1 - 1)", 1.0);
    }
    #[test]
    fn comparison_loosest() {
        assert!(test_bool("1 + 2 > 2"));
    }
}

#[allow(warnings)]
fn test_parser(src: &str) -> Answer {
    Parser::new(src.to_string()).equation().unwrap()
}

#[allow(warnings)]
fn test_num(src: &str) -> f64 {
    test_parser(src).get::<f64>().unwrap()
}

#[allow(warnings)]
fn test_bool(src: &str) -> bool {
    test_parser(src).get::<bool>().unwrap()
}

#[allow(warnings)]
fn assert_close(src: &str, expected: f64) {
    let got = test_num(src);

    assert!(
        (got - expected).abs() < 1e-9,
        "'{src}': expected {expected}, got {got}"
    )
}
