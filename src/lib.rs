use num_bigint::{BigUint, RandBigInt};
use rand::rngs::ThreadRng;

// Calculate n^exp mod p
pub fn mod_exp(n: &BigUint, exp: &BigUint, prime: &BigUint) -> BigUint {
    n.modpow(exp, prime)
}

// Solve for s: s = k - c * x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q); //q^1
    }
    return q - (c * x - k).modpow(&BigUint::from(1u32), q); //k cannot be < c * x
}

// Verify conditions
pub fn verify(
    r1: &BigUint,
    r2: &BigUint,
    y1: &BigUint,
    y2: &BigUint,
    alpha: &BigUint,
    beta: &BigUint,
    c: &BigUint,
    s: &BigUint,
    p: &BigUint,
) -> bool {
    // Verify r1 = (alpha^s * y1^c) mod p
    let condition_one = r1 == &(alpha.modpow(s, p) * y1.modpow(c, p) % p);

    // Verify r2 = (beta^s * y2^c) mod p
    let condition_two = r2 == &(beta.modpow(s, p) * y2.modpow(c, p) % p);

    // Both conditions must be true
    condition_one && condition_two
}

//Generate random numbers
pub fn gen_ran_below(bound: &BigUint) -> BigUint {
    let mut ran_num: ThreadRng = rand::thread_rng();

    ran_num.gen_biguint_below(bound)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proto_test() {
        let alpha: BigUint = BigUint::from(4u32); //generator
        let beta: BigUint = BigUint::from(9u32); //generator
        let p: BigUint = BigUint::from(23u32); //prime
        let q: BigUint = BigUint::from(11u32); //order # elements

        let x: BigUint = BigUint::from(6u32); //Secret
        let k: BigUint = BigUint::from(7u32); //random constant prover

        let c: BigUint = BigUint::from(4u32); //random constant verifier

        //Prover to verifier set 1
        let y1: BigUint = mod_exp(&alpha, &x, &p);
        let y2: BigUint = mod_exp(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        //Prover to verifier set 2
        let r1: BigUint = mod_exp(&alpha, &k, &p);
        let r2: BigUint = mod_exp(&beta, &k, &p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        //compute S
        let s: BigUint = solve(&k, &c, &x, &q);
        assert_eq!(s, BigUint::from(5u32));

        //Solve
        let result: bool = verify(&r1, &r2, &y1, &y2, &alpha, &beta, &c, &s, &p);
        assert!(result)
    }

    #[test]
    fn proto_test_random_numbers() {
        let alpha: BigUint = BigUint::from(4u32); //generator
        let beta: BigUint = BigUint::from(9u32); //generator
        let p: BigUint = BigUint::from(23u32); //prime
        let q: BigUint = BigUint::from(11u32); //order # elements

        let x: BigUint = BigUint::from(6u32); //Secret
        let k: BigUint = gen_ran_below(&q); //random constant prover

        let c: BigUint = gen_ran_below(&q); //random constant verifier

        //Prover to verifier set 1
        let y1: BigUint = mod_exp(&alpha, &x, &p);
        let y2: BigUint = mod_exp(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        //Prover to verifier set 2
        let r1: BigUint = mod_exp(&alpha, &k, &p);
        let r2: BigUint = mod_exp(&beta, &k, &p);

        //compute S
        let s: BigUint = solve(&k, &c, &x, &q);

        //Solve
        let result: bool = verify(&r1, &r2, &y1, &y2, &alpha, &beta, &c, &s, &p);
        assert!(result)
    }
}
