pub mod core;

#[cfg(test)]
const K_REPLICATIONS: usize = 4;
#[cfg(not(test))]
const K_REPLICATIONS: usize = 20;
const ALPHA_PARALLEL: usize = 3;
