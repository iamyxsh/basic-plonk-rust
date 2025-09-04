#[cfg(test)]
mod tests;

use ark_bls12_381::{Bls12_381, Fr, G1Affine, G1Projective};
use ark_ec::{AffineRepr, CurveGroup, VariableBaseMSM, pairing::Pairing};
use ark_ff::Field;
use ark_poly::{DenseUVPolynomial, Polynomial, univariate::DensePolynomial};
use setup::TrustedSetup;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commitment(pub G1Affine);

#[derive(Clone, Debug)]
pub struct OpeningProof(pub G1Affine);

pub fn commit(setup: &TrustedSetup, poly: &DensePolynomial<Fr>) -> Commitment {
    let coeffs = poly.coeffs();

    assert!(
        coeffs.len() <= setup.g1_powers.len(),
        "Polynomial degree {} exceeds setup max degree {}",
        coeffs.len() - 1,
        setup.max_degree()
    );

    let result = G1Projective::msm_unchecked(&setup.g1_powers[..coeffs.len()], coeffs);

    Commitment(result.into_affine())
}

pub fn open(setup: &TrustedSetup, poly: &DensePolynomial<Fr>, z: Fr, y: Fr) -> OpeningProof {
    let mut numerator_coeffs = poly.coeffs().to_vec();
    if numerator_coeffs.is_empty() {
        numerator_coeffs.push(-y);
    } else {
        numerator_coeffs[0] -= y;
    }
    let numerator = DensePolynomial::from_coefficients_vec(numerator_coeffs);

    let quotient = divide_by_linear(&numerator, z);

    let proof_point = commit(setup, &quotient);

    OpeningProof(proof_point.0)
}

fn divide_by_linear(poly: &DensePolynomial<Fr>, root: Fr) -> DensePolynomial<Fr> {
    let coeffs = poly.coeffs();

    if coeffs.is_empty() {
        return DensePolynomial::from_coefficients_vec(vec![]);
    }

    if coeffs.len() == 1 {
        return DensePolynomial::from_coefficients_vec(vec![]);
    }

    let n = coeffs.len();
    let mut quotient_coeffs = vec![Fr::from(0u64); n - 1];

    quotient_coeffs[n - 2] = coeffs[n - 1];

    for i in (0..n - 2).rev() {
        quotient_coeffs[i] = coeffs[i + 1] + root * quotient_coeffs[i + 1];
    }

    DensePolynomial::from_coefficients_vec(quotient_coeffs)
}

pub fn verify(
    setup: &TrustedSetup,
    commitment: &Commitment,
    z: Fr,
    y: Fr,
    proof: &OpeningProof,
) -> bool {
    let y_g1 = setup.g1_powers[0] * y;
    let c_minus_y: G1Affine = (commitment.0.into_group() - y_g1).into_affine();

    let z_g2 = setup.g2_gen * z;
    let tau_minus_z = (setup.g2_tau.into_group() - z_g2).into_affine();

    let lhs = Bls12_381::pairing(c_minus_y, setup.g2_gen);
    let rhs = Bls12_381::pairing(proof.0, tau_minus_z);

    lhs == rhs
}
