use num_bigint::BigUint;

// Calculate n^exp mod p
pub fn mod_exp(n: &BigUint, exp: &BigUint, p: &BigUint) -> BigUint {
    n.modpow(exp, p)
}

// Solve for s: s = k - c * x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q); //q^1
    }
    return q - (c * x - k).modpow(&BigUint::from(1u32), q); //k cannot be < c * x
}

//Verify conditions
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
    //r1 = alpha^s * y1^c
    let condition_one = *r1 == alpha.modpow(s, p) * y1.modpow(c, p);

    //r2 = beta^s * y2^c
    let condition_two = *r2 == beta.modpow(s, p) * y2.modpow(c, p);

    //both must be true
    condition_one && condition_two
}
