use hex;
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

    #[test] //https://www.rfc-editor.org/rfc/rfc5114#section-2.1
            //1024-bit MODP Group with 160-bit Prime Order Subgroup
    fn proto_test_1024_bit_constants() {
        //The generator generates a prime-order subgroup of size:
        //q = F518AA87 81A8DF27 8ABA4E7D 64B7CB9D 49462353
        let p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap(),);
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );
        let alpha = BigUint::from_bytes_be(&hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(),);

        //G is order q(prime) then alpha^i is a generator
        let beta: BigUint = alpha.modpow(&ZKP::gen_ran_below(&q), &p);

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x: BigUint = ZKP::gen_ran_below(&q); //Secret
        let k: BigUint = ZKP::gen_ran_below(&q); //random constant prover

        let c: BigUint = ZKP::gen_ran_below(&q); //random constant verifier

        //Prover to verifier set 1
        let y1: BigUint = ZKP::mod_exp(&alpha, &x, &p);
        let y2: BigUint = ZKP::mod_exp(&beta, &x, &p);

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
