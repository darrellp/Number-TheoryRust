use crate::utilities::nt_error;
use crate::utilities::numeric_trait::Numeric;
use crate::utilities::utilities;

pub fn power<T: Numeric>(x: T, n: T, modulo: T) -> Result<T, nt_error::NtError> {
    if n == T::zero() {
        return Ok(T::one());
    }

    if n < T::zero() {
        return Err(nt_error::NtError::BadArgument);
    }
    let mut mask = utilities::top_bit_mask(n);
    let mut res = T::one();
    while mask != T::zero() {
        let opt = res.checked_mul(&res);
        res = match opt {
            None => return Err(nt_error::NtError::Overflow),
            Some(val) => val % modulo,
        };

        if (mask & n) != T::zero() {
            let opt = res.checked_mul(&x);
            res = match opt {
                None => return Err(nt_error::NtError::Overflow),
                Some(val) => val % modulo,
            };
        }
        mask = mask >> 1;
    }
    Ok(res)
}
