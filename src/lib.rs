use num_bigint::{BigUint, RandBigInt};
use rand::rngs::ThreadRng;

pub struct ZKP {
    p: BigUint,     //prime
    q: BigUint,     //group order
    alpha: BigUint, //generator
    beta: BigUint,  //generator
}

impl ZKP {
    // Calculate n^exp mod p
    pub fn mod_exp(n: &BigUint, exp: &BigUint, prime: &BigUint) -> BigUint {
        n.modpow(exp, prime)
    }

    // Solve for s: s = k - c * x mod q
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            return (k - c * x).modpow(&BigUint::from(1u32), &self.q); //q^1
        }
        return &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q); //k cannot be < c * x
    }

    // Verify conditions
    pub fn verify(
        &self,
        r1: &BigUint,
        r2: &BigUint,
        y1: &BigUint,
        y2: &BigUint,
        c: &BigUint,
        s: &BigUint,
    ) -> bool {
        // Verify r1 = (alpha^s * y1^c) mod p
        let condition_one =
            r1 == &(&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p) % &self.p);

        // Verify r2 = (beta^s * y2^c) mod p
        let condition_two =
            r2 == &(&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p) % &self.p);

        // Both conditions must be true
        condition_one && condition_two
    }

    //Generate random numbers
    pub fn gen_ran_below(bound: &BigUint) -> BigUint {
        let mut ran_num: ThreadRng = rand::thread_rng();

        ran_num.gen_biguint_below(bound)
    }
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

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x: BigUint = BigUint::from(6u32); //Secret
        let k: BigUint = BigUint::from(7u32); //random constant prover

        let c: BigUint = BigUint::from(4u32); //random constant verifier

        //Prover to verifier set 1
        let y1: BigUint = ZKP::mod_exp(&alpha, &x, &p);
        let y2: BigUint = ZKP::mod_exp(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        //Prover to verifier set 2
        let r1: BigUint = ZKP::mod_exp(&alpha, &k, &p);
        let r2: BigUint = ZKP::mod_exp(&beta, &k, &p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        //compute S
        let s: BigUint = zkp.solve(&k, &c, &x);
        assert_eq!(s, BigUint::from(5u32));

        //Solve
        let result: bool = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result)
    }

    #[test]
    fn proto_test_random_numbers() {
        let alpha: BigUint = BigUint::from(4u32); //generator
        let beta: BigUint = BigUint::from(9u32); //generator
        let p: BigUint = BigUint::from(23u32); //prime
        let q: BigUint = BigUint::from(11u32); //order # elements

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x: BigUint = BigUint::from(6u32); //Secret
        let k: BigUint = ZKP::gen_ran_below(&q); //random constant prover

        let c: BigUint = ZKP::gen_ran_below(&q); //random constant verifier

        //Prover to verifier set 1
        let y1: BigUint = ZKP::mod_exp(&alpha, &x, &p);
        let y2: BigUint = ZKP::mod_exp(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        //Prover to verifier set 2
        let r1: BigUint = ZKP::mod_exp(&alpha, &k, &p);
        let r2: BigUint = ZKP::mod_exp(&beta, &k, &p);

        //compute S
        let s: BigUint = zkp.solve(&k, &c, &x);

        //Solve
        let result: bool = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result)
    }

    //zkp instead of ZKP when use method param //.clone create copy for usage
}
