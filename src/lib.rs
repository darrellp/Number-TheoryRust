pub mod number_theory {
    pub mod euclidean;
    pub mod power_mod;
}

pub mod utilities {
    pub mod nt_error;
    pub mod u_to_i;
    pub(crate) mod numeric_trait;
    pub(crate) mod utilities;
}

#[cfg(test)]
mod tests;
