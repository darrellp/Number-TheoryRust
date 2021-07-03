use crate::numeric_trait::Numeric;
use num::{abs, FromPrimitive};

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

/**
Returns a GCD of two numbers and the linear coefficients to produce that GCD from the two numbers

# Arguments

* `val1` - First number
* `val2` - Second number

# Returns
* Tuple whose first value is GCD, second value is coeff for val1 , third value is coeff for val2

# Examples

```
let (gcd, coeff1, coeff2) = number_theory::euclidean_extension::calc_euclidean_ext(97, 18);
assert_eq!(gcd, 1);
assert_eq!(coeff1 * 97 + coeff2 * 18, gcd);
```
*/
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

/**
Returns a function which produces coefficients to solve ax + by = c if any exist - also GCD(a, b)

# Arguments

* `a`, `b`, `c` - Coefficients in linear equation ax + by = c

# Returns
* Option with a closure which takes an i32 and each different value produces different x, y
  to solve the above equation.  The function provides a "small" pair for i = 0.  If there
  is no solution then Option.None is returned.

# Examples

```
    let (opt, gcd) = number_theory::euclidean_extension::solve_diophantine(7, 13, 5);
    assert_eq!(gcd, 1);
    let fn_solve = opt.unwrap();
    let (x, y) = fn_solve(0);
    assert_eq!(7 * x + 13 * y, 5);
    let (x1, y1) = fn_solve(1);
    assert_ne!(x, x1);
    assert_eq!(7*x1 + 13 * y1, 5);


```
*/
pub fn solve_diophantine<T>(a: T, b: T, c: T) -> (Option<impl Fn(i32) -> (T, T)>, T)
where
    T: Numeric,
{
    let zero = FromPrimitive::from_usize(0).unwrap();
    let (gcd, c1, c2) = calc_euclidean_ext(a, b);
    if c % gcd != zero {
        return (None, gcd);
    }
    let cnst1 = c * c1 / gcd;
    let cnst2 = c * c2 / gcd;
    let cf1 = b / gcd;
    let cf2 = -a / gcd;

    let q = if abs(cnst1) > abs(cnst2) {
        cnst1 / cf1
    } else {
        cnst2 / cf2
    };

    let cnst1 = cnst1 - q * cf1;
    let cnst2 = cnst2 - q * cf2;

    let result = move |i: i32| {
        let i_t = FromPrimitive::from_i32(i).unwrap();
        (cf1 * i_t + cnst1, cf2 * i_t + cnst2)
    };

    (Some(result), gcd)
}

//pub fn SolveLinearCongruence<T>(T a, T b, T mod)
