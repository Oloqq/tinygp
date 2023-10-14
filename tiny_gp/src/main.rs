#![allow(dead_code, unused_variables)]

mod params;
mod tinygp;

use std::error::Error;
use tinygp::TinyGP;

type Berror = Box<dyn Error>;

fn main() {
    let seed: Option<u64> = Some(3);
    let filename = "../linear.dat";

    let mut tgp = TinyGP::from_problem(filename, seed.unwrap()).unwrap();
    tgp.evolve(100);
}
