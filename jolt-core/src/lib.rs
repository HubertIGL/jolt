#![allow(non_snake_case)]
#![allow(clippy::assertions_on_result_states)]
#![allow(clippy::needless_range_loop)]
#![feature(extend_one)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod benches;
pub mod lasso;
mod msm;
pub mod poly;
pub mod r1cs;
pub mod subprotocols;
pub mod jolt;
pub mod utils;

#[cfg(test)]
mod e2e_test;
