use crate::utilities::nt_error;
use crate::utilities::numeric_trait::Numeric;
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
Returns a GCD of two numbers

# Arguments

* `val1` - First number
* `val2` - Second number

# Returns
* GCD

# Examples

```
use number_theory::utilities::nt_error;
use number_theory::utilities::numeric_trait::Numeric;
use number_theory::number_theory::euclidean;

let gcd = euclidean::gcd(97, 18);
assert_eq!(gcd, 1);
```
*/
pub fn gcd<T>(val1: T, val2: T) -> T
where
    T: Numeric,
{
    let zero: T = FromPrimitive::from_i32(0).unwrap();
    let mut val1 = abs(val1);
    let mut val2 = abs(val2);

    while val2 != zero {
        let r = val1 % val2;
        val1 = val2;
        val2 = r;
    }
    val1
}

/**
Returns a LCM of two numbers

# Arguments

* `val1` - First number
* `val2` - Second number

# Returns
* LCM

# Examples

```
use number_theory::utilities::nt_error;
use number_theory::utilities::numeric_trait::Numeric;
use number_theory::number_theory::euclidean;

let lcm = euclidean::lcm(4, 6)
    .unwrap_or_else(|_| panic!("Failed!"));
assert_eq!(lcm, 12);
```
*/
pub fn lcm<T>(val1: T, val2: T) -> Result<T, nt_error::NtError>
where
    T: Numeric,
{
    let opt = val1.checked_mul(&val2);
    match opt {
        None => Err(nt_error::NtError::Overflow),
        Some(val) => Ok(val / gcd(val1, val2)),
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
use number_theory::utilities::nt_error;
use number_theory::utilities::numeric_trait::Numeric;
use number_theory::number_theory::euclidean;

let (gcd, coeff1, coeff2) = euclidean::calc_euclidean_ext(97, 18);
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
use number_theory::utilities::nt_error;
use number_theory::utilities::numeric_trait::Numeric;
use number_theory::number_theory::euclidean;

let (fn_solve, gcd) = euclidean::solve_diophantine(7, 13, 5)
    .unwrap_or_else(|_| panic!("Failed!"));
assert_eq!(gcd, 1);
let (x, y) = fn_solve(0);
assert_eq!(7 * x + 13 * y, 5);
let (x1, y1) = fn_solve(1);
assert_ne!(x, x1);
assert_eq!(7*x1 + 13 * y1, 5);


```
*/
pub fn solve_diophantine<T>(
    a: T,
    b: T,
    c: T,
) -> Result<(impl Fn(i32) -> (T, T), T), nt_error::NtError>
where
    T: Numeric,
{
    let zero = FromPrimitive::from_usize(0).unwrap();
    let (gcd, c1, c2) = calc_euclidean_ext(a, b);
    if c % gcd != zero {
        return Err(nt_error::NtError::NoSolns);
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

    Ok((result, gcd))
}

/**
Returns solutions to ax = b (mod modulo)

# Arguments

* `a`, `b`, `modulo` - Coefficients in ax = b (mod modulo)

# Returns
* Optional vector of solutions mod modulo of length GCD(a, b)

# Examples

```
use number_theory::utilities::nt_error;
use number_theory::utilities::numeric_trait::Numeric;
use number_theory::number_theory::euclidean;

let big_a = 123;
let big_b = 9123;
let big_mod = 321123;

let solns = euclidean::solve_linear_congruence(big_a, big_b, big_mod)
    .unwrap_or_else(|_| panic!("failed"));
assert_eq!(solns.len(), 3);
for isoln in solns {
    assert_eq!(big_b, big_a * isoln % big_mod);
}
```
*/
pub fn solve_linear_congruence<T>(a: T, b: T, modulo: T) -> Result<Vec<T>, nt_error::NtError>
where
    T: Numeric,
{
    let zero = FromPrimitive::from_usize(0).unwrap();
    let one = FromPrimitive::from_usize(1).unwrap();

    let (func_solns, gcd) = match solve_diophantine(a, modulo, b) {
        Err(e) => return Err(e),
        Ok(val) => val,
    };
    let mut ret = Vec::new();
    let gcd_int_opt = num::ToPrimitive::to_i32(&gcd);
    let gcd_int = match gcd_int_opt {
        None => return Err(nt_error::NtError::Overflow),
        Some(val) => val,
    };

    for i in 0..gcd_int {
        let (sln, _) = func_solns(i);
        let sln = if sln > modulo {
            sln - (sln / modulo) * modulo
        } else if sln < zero {
            sln + ((modulo - one - sln) / modulo) * modulo
        } else {
            sln
        };
        ret.push(sln);
    }
    Ok(ret)
}

/**
Returns a^-1 (mod modulo)

# Arguments

* `a`, `modulo` - Coefficients in a^-1 (mod modulo)

# Returns
* Some(Inverse of a mod modulo) or None if there is no inverse

# Examples

```
use number_theory::utilities::nt_error;
use number_theory::utilities::numeric_trait::Numeric;
use number_theory::number_theory::euclidean;

let inverse = euclidean::inverse_mod(3, 11)
    .unwrap_or_else(|_| panic!("Failed"));
assert_eq!(inverse, 4);
```
*/
pub fn inverse_mod<T>(n: T, modulo: T) -> Result<T, nt_error::NtError>
where
    T: Numeric,
{
    match solve_linear_congruence(n, FromPrimitive::from_usize(1).unwrap(), modulo) {
        Err(e) => Err(e),
        Ok(ret) => Ok(ret[0]),
    }
}
