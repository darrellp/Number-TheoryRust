use crate::numeric_trait::Numeric;
use num::FromPrimitive;

struct ExtMatrix<T>
where
    T: Numeric,
{
    first_row: usize,
    mtx: [[T; 3]; 3],
}

impl<T> ExtMatrix<T>
where
    T: Numeric,
{
    fn new(val1: T, val2: T) -> ExtMatrix<T> {
        let zero: T = FromPrimitive::from_i32(0).unwrap();
        let one: T = FromPrimitive::from_i32(1).unwrap();

        let mtx = [[val1, one, zero], [val2, zero, one], [zero, zero, zero]];
        ExtMatrix { first_row: 0, mtx }
    }

    fn step(&mut self) {
        let next_row: usize = (self.first_row + 1) % 3;
        let spare_row: usize = (self.first_row + 2) % 3;
        let q = self.mtx[self.first_row][0] / self.mtx[next_row][0];
        for i in 0..3 {
            self.mtx[spare_row][i] = self.mtx[self.first_row][i] - q * self.mtx[next_row][i];
        }
        self.first_row = (self.first_row + 1) % 3;
    }

    fn finished(&self) -> bool {
        self.mtx[(self.first_row + 1) % 3][0] == FromPrimitive::from_usize(0).unwrap()
    }

    fn result(&self) -> (T, T, T) {
        let row = &self.mtx[self.first_row];
        (row[0], row[1], row[2])
    }
}

/// Returns a GCD of two numbers and the linear coefficients to produce that GCD from the two numbers
///
/// # Arguments
///
/// * `val1` - First number
/// * `val2` - Second number
///
/// # Returns
/// * Tuple with first value GCD, second value is coeff for val1 , third value is coeff for val2
/// # Examples
///
/// ```
/// let (gcd, coeff1, coeff2) = number_theory::euclidean_extension::calc_euclidean_ext(97, 18);
/// assert_eq!(gcd, 1);
/// assert_eq!(coeff1 * 97 + coeff2 * 18, gcd);
/// ```
pub fn calc_euclidean_ext<T>(val1: T, val2: T) -> (T, T, T)
where
    T: Numeric,
{
    let mut mtx = ExtMatrix::new(val1, val2);
    while !mtx.finished() {
        mtx.step();
    }
    mtx.result()
}
