use num::FromPrimitive;
use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    usize,
};

pub trait Numeric:
    Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
    + Copy
    + PartialEq
    + FromPrimitive
{
}

impl<T> Numeric for T where
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Rem<Output = T>
        + Copy
        + PartialEq
        + FromPrimitive
{
}
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
