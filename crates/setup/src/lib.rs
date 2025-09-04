use ark_bls12_381::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::PrimeGroup;
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{Field, UniformRand};
use rand::thread_rng;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct TrustedSetup {
    pub g1_powers: Vec<G1Affine>,

    pub g2_gen: G2Affine,

    pub g2_tau: G2Affine,
}

impl TrustedSetup {
    pub fn generate(max_degree: usize) -> Self {
        let mut rng = thread_rng();

        let tau: Fr = Fr::rand(&mut rng);

        let g1_generator = G1Projective::generator();
        let g2_generator = G2Projective::generator();

        let mut g1_powers = Vec::with_capacity(max_degree + 1);
        let mut current_tau_power = Fr::from(1u64);

        for _ in 0..=max_degree {
            let point = g1_generator * current_tau_power;
            g1_powers.push(point.into_affine());
            current_tau_power *= tau;
        }

        let g2_tau = (g2_generator * tau).into_affine();

        TrustedSetup {
            g1_powers,
            g2_gen: g2_generator.into_affine(),
            g2_tau,
        }
    }

    pub fn max_degree(&self) -> usize {
        self.g1_powers.len() - 1
    }
}
