use crate::euclidean_extension::calc_euclidean_ext;
use crate::euclidean_extension::solve_diophantine;

// Euclidean Extension
#[test]
fn euclidean_extension() {
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
    let (opt, gcd) = solve_diophantine(7, 13, 5);
    assert_eq!(gcd, 1);
    let fn_solve = opt.unwrap();
    let (x, y) = fn_solve(0);
    assert_eq!(7 * x + 13 * y, 5);
}
