use ark_bls12_381::Fr;
use ark_ff::{Field, One, Zero};

#[derive(Clone, Debug)]
pub struct Gate {
    /// Left wire selector
    pub q_l: Fr,
    /// Right wire selector
    pub q_r: Fr,
    /// Output wire selector
    pub q_o: Fr,
    /// Multiplication selector
    pub q_m: Fr,
    /// Constant selector
    pub q_c: Fr,
}

impl Gate {
    /// Creates a multiplication gate: a * b = c
    ///
    /// Gate equation: 0·a + 0·b + (-1)·c + 1·(a·b) + 0 = 0
    /// Simplifies to: a·b - c = 0, i.e., a·b = c
    pub fn multiplication() -> Self {
        Gate {
            q_l: Fr::zero(),
            q_r: Fr::zero(),
            q_o: -Fr::one(),
            q_m: Fr::one(),
            q_c: Fr::zero(),
        }
    }

    /// Creates an addition gate: a + b = c
    ///
    /// Gate equation: 1·a + 1·b + (-1)·c + 0·(a·b) + 0 = 0
    /// Simplifies to: a + b - c = 0, i.e., a + b = c
    #[allow(dead_code)]
    pub fn addition() -> Self {
        Gate {
            q_l: Fr::one(),
            q_r: Fr::one(),
            q_o: -Fr::one(),
            q_m: Fr::zero(),
            q_c: Fr::zero(),
        }
    }

    /// Check if wire values satisfy this gate's constraint.
    ///
    /// Returns true if qL·a + qR·b + qO·c + qM·(a·b) + qC = 0
    pub fn is_satisfied(&self, a: Fr, b: Fr, c: Fr) -> bool {
        let result = self.q_l * a + self.q_r * b + self.q_o * c + self.q_m * (a * b) + self.q_c;

        result.is_zero()
    }
}
