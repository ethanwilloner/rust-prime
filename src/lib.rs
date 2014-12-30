extern crate num;

use num::bigint::{BigUint, RandBigInt};
use num::integer::{Integer};
use num::One;
use std::rand;

pub mod prime;

pub fn gen_prime(bitsize: uint) -> BigUint {
    let one: BigUint = One::one();
    let two: BigUint = &one + &one;
    let mut rng = rand::task_rng();
    let mut n: BigUint = rng.gen_biguint(bitsize);

    if n.is_even() {
        n = n + &one;
    }

    loop {
        if prime::miller_rabin(&n, bitsize) {
            break;
        }
        else {
            n = n + &two;
        }
    }
    return n;
}
