use crate::number_theory::euclidean::calc_euclidean_ext;
use crate::number_theory::euclidean::solve_diophantine;
use crate::number_theory::euclidean::solve_linear_congruence;
use crate::number_theory::power_mod::power;

// Euclidean Extension
#[test]
fn euclidean_test() {
    let (gcd, coeff1, coeff2) = calc_euclidean_ext(97, 18);
    assert_eq!(gcd, 1);
    assert_eq!(coeff1, -5);
    assert_eq!(coeff2, 27);
    let (gcd, coeff1, coeff2) = calc_euclidean_ext(541, 7919);
    assert_eq!(gcd, 1);
    assert_eq!(coeff1, -1010);
    assert_eq!(coeff2, 69);
}

#[test]
fn solve_diophantine_test() {
    let (fn_solve, gcd) = solve_diophantine(7, 13, 5).unwrap_or_else(|_| panic!("failed!"));
    assert_eq!(gcd, 1);
    let (x, y) = fn_solve(0);
    assert_eq!(7 * x + 13 * y, 5);
}

#[test]
fn solve_linear_congruence_test() {
    let big_a: i128 = 6123123;
    let big_b: i128 = 6123123123;
    let big_mod: i128 = 9123123123123;
    // let big_a = apint::Int::from_i128(6123123i128);
    // let big_b = apint::Int::from_i128(6123123123);
    // let big_mod = apint::Int::from_i128(9123123123123);

    let solns =
        solve_linear_congruence(big_a, big_b, big_mod).unwrap_or_else(|_| panic!("Failed!"));
    assert_eq!(solns.len(), 3);
    for isoln in solns {
        assert_eq!(big_b, big_a * isoln % big_mod);
    }
}

#[test]
fn power_test() {
    let val = power(2357, 2357, 3599).unwrap_or_else(|_| panic!("Failed!"));
    assert_eq!(3115_i64, val);
}
