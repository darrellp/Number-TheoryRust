use crate::utilities::numeric_trait::Numeric;
use num::ToPrimitive;

static LMOB_MAPPING: [usize; 16] = [0, 0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3];

pub fn top_bit_mask<T: Numeric>(n: T) -> T {
    if n == T::zero() {
        T::zero()
    } else {
        T::one() << leftmost_one_index(n)
    }
}

/**
Returns the index of the leftmost one bit in a number

# Arguments

* `n` - value

# Returns
* Position of the leftmost one bit with the 2^0 digit being digit 0 and counting
* left
*/
pub fn leftmost_one_index<T: Numeric>(n: T) -> usize {
    let mut bit_count = std::mem::size_of::<T>() * 8;
    let mut ret = 0;

    if n < T::zero() {
        return bit_count;
    }
    let mut mut_n = n;
    while bit_count > 4 {
        bit_count = bit_count >> 1;
        let shifted = mut_n >> bit_count;
        if shifted != T::zero() {
            ret += bit_count;
            mut_n = shifted;
        }
    }
    let n = ToPrimitive::to_usize(&mut_n).unwrap();
    ret + LMOB_MAPPING[n]
}
