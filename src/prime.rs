use num::bigint::{BigUint, RandBigInt};
use num::integer::{Integer};
use num::{Zero, One};
use std::rand;

pub fn miller_rabin(p: &BigUint, bitsize: uint) -> bool{
    let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
    let two: BigUint = &one + &one;
    let (mut s, d): (BigUint, BigUint) = write_n_minus_one(p);
    let mut rng = rand::task_rng();
    
    // These Miller-Rabin tolerances were gotten from the 
    // Handbook of Applied Cryptography, as per OpenSSL's
    // implementation of the algorithm
    let t: uint = {
        if bitsize >= 1300u {
            2
        } else if bitsize >= 850 {
            3
        } else if bitsize >= 650 {
            4
        } else if bitsize >= 550 {
            5
        } else if bitsize >= 450 {
            6        
        } else if bitsize >= 400 {
            7
        } else if bitsize >= 350 {
            8
        } else if bitsize >= 300 {
            9
        } else if bitsize >= 250 {
            12
        } else if bitsize >= 200 {
            15
        } else if bitsize >= 150 {
            18
        } else if bitsize >= 100 {
            27
        } else {
            27
        }
    };

    'WitnessLoop: for _ in range(0, t) {
        let a = rng.gen_biguint_range(&two, p);
        let mut x = mod_exp(&a, &d, p);
        if (x == one) || (x == (p - &one)) {
            continue 'WitnessLoop;
        }
        while (&s).sub(&one) > zero {
            x = (&x).mul(&x).rem(p);
            if x == one {
                return false;
            }
            if x == (p - &one) {
                continue 'WitnessLoop;
            }
            s = (&s).sub(&one);
        }
        return false;
    }
    return true;
}

fn write_n_minus_one(p: &BigUint) -> (BigUint, BigUint){
    let one: BigUint = One::one();
    let two: BigUint = &one + &one;
    let mut d: BigUint = p.clone().sub(&one);
    let mut s: BigUint = Zero::zero();
    while d.is_even() {
        d = d.div(&two);
        s = s + &one;
    }
    return (s,d)
}

fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint{
    let mut b: BigUint = base.clone();
    let mut e: BigUint = exp.clone();
    let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
    let mut result: BigUint = one.clone();
    while e > zero {
        if e.is_odd() {
            result = result.rem(modulus).mul((&b).rem(modulus)).rem(modulus);
        }
        e = e.shr(1u);
        b = ((&b).rem(modulus)).mul((&b).rem(modulus)).rem(modulus);
    }
    return result;
}

#[cfg(test)]
mod tests {
    extern crate num;
    use super::{mod_exp, miller_rabin};
    use num::bigint::{BigUint, ToBigUint};
    use std::str::FromStr;

    #[test]
    fn test_mod_exp() {
        let five = 5u.to_biguint().unwrap();
        let ten = 10u.to_biguint().unwrap();
        let fifteen = 15u.to_biguint().unwrap();

        assert_eq!(mod_exp(&five, &ten, &fifteen), ten);
        assert_eq!(mod_exp(&ten, &five, &fifteen), ten);
    }

    #[test]
    fn test_mod_exp_big() {
        let big_base: BigUint = FromStr::from_str("12345678901234567890123456789000").unwrap();
        let big_exp: BigUint = FromStr::from_str("12345678901234567890123456789000").unwrap();
        let big_mod: BigUint = FromStr::from_str("98765432109876543210987654321000").unwrap();
        let big_out: BigUint = FromStr::from_str("29647215002964721500296472150000").unwrap();
        assert_eq!(mod_exp(&big_base, &big_exp, &big_mod), big_out);
    }
    
    #[test]
    fn test_miller_rabin_composite() {
        let known_composite = FromStr::from_str("170141183460469231731687303715884105725").unwrap();    
        assert_eq!(miller_rabin(&known_composite, 256u), false);
    }
    
    #[test]
    fn test_miller_rabin_prime() {
        let known_prime = FromStr::from_str("170141183460469231731687303715884105727").unwrap();    
        assert_eq!(miller_rabin(&known_prime, 256u), true);
    }
}

#[cfg(test)]
mod bench {
    extern crate num;
    extern crate test;
    use self::test::Bencher;
    use super::{mod_exp, miller_rabin};
    use num::bigint::{BigUint};
    use std::str::FromStr;

    #[bench]
    fn bench_mod_exp_32_byte(b: &mut Bencher){
        let big_base: BigUint = FromStr::from_str("12345678901234567890123456789000").unwrap();
        let big_exp: BigUint = FromStr::from_str("12345678901234567890123456789000").unwrap();
        let big_mod: BigUint = FromStr::from_str("98765432109876543210987654321000").unwrap();
        b.iter( || {
            mod_exp(&big_base, &big_exp, &big_mod)
        });
    }
    
    #[bench]
    fn bench_miller_rabin_prime(b: &mut Bencher) {
        let known_prime = FromStr::from_str("170141183460469231731687303715884105727").unwrap();    
        b.iter( || {
            miller_rabin(&known_prime, 256u);
        });
    }
}
