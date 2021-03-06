#![feature(macro_rules)]

extern crate serialize;
extern crate simplot;
extern crate test;

pub use bencher::Bencher;
pub use criterion::Criterion;

mod bencher;
mod criterion;
mod fs;
mod math;
mod outliers;
mod plot;
mod statistics;
mod stream;
mod target;
mod time;
