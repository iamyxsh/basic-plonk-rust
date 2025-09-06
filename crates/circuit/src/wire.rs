use ark_bls12_381::Fr;

#[derive(Clone, Debug)]
pub struct Witness {
    pub x: Fr,
}

impl Witness {
    pub fn new(x: u64) -> Self {
        Witness { x: Fr::from(x) }
    }

    pub fn new_signed(x: i64) -> Self {
        if x >= 0 {
            Witness {
                x: Fr::from(x as u64),
            }
        } else {
            Witness {
                x: -Fr::from((-x) as u64),
            }
        }
    }
}
