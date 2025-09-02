use ark_bls12_381::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{Field, UniformRand};
use rand::rng;

#[derive(Clone, Debug)]
pub struct TrustedSetup {
    pub g1_powers: Vec<G1Affine>,

    pub g2_gen: G2Affine,

    pub g2_tau: G2Affine,
}

impl TrustedSetup {
    pub fn generate(max_degree: usize) -> Self {
        let mut rng = rng();

        let tau: Fr = Fr::rand(&mut rng);
    }
}
