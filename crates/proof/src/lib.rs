use ark_bls12_381::Fr;
use ark_ff::{Field, One};
use ark_poly::{DenseUVPolynomial, Polynomial, univariate::DensePolynomial};
use circuit::{square_circuit::SquareCircuit, witness::Witness};
use kzg::{Commitment, OpeningProof, commit, open};
use setup::TrustedSetup;

#[derive(Clone, Debug)]
pub struct Proof {
    pub a_comm: Commitment,
    pub b_comm: Commitment,
    pub c_comm: Commitment,

    pub q_comm: Commitment,

    pub a_eval: Fr,
    pub b_eval: Fr,
    pub c_eval: Fr,
    pub q_eval: Fr,

    pub a_opening: OpeningProof,
    pub b_opening: OpeningProof,
    pub c_opening: OpeningProof,
    pub q_opening: OpeningProof,
}

pub fn prove(setup: &TrustedSetup, circuit: &SquareCircuit, witness: &Witness) -> Proof {
    let wires = circuit.wire_values(witness);

    let a_poly = DensePolynomial::from_coefficients_vec(vec![wires.a]);
    let b_poly = DensePolynomial::from_coefficients_vec(vec![wires.b]);
    let c_poly = DensePolynomial::from_coefficients_vec(vec![wires.c]);

    let a_comm = commit(setup, &a_poly);
    let b_comm = commit(setup, &b_poly);
    let c_comm = commit(setup, &c_poly);

    let t_poly = compute_constraint_polynomial(&a_poly, &b_poly, &c_poly);

    let vanishing_root = Fr::one();
    let q_poly = divide_by_vanishing(&t_poly, vanishing_root);

    let q_comm = commit(setup, &q_poly);

    let zeta = Fr::from(7u64);

    let a_eval = a_poly.evaluate(&zeta);
    let b_eval = b_poly.evaluate(&zeta);
    let c_eval = c_poly.evaluate(&zeta);
    let q_eval = q_poly.evaluate(&zeta);

    let a_opening = open(setup, &a_poly, zeta, a_eval);
    let b_opening = open(setup, &b_poly, zeta, b_eval);
    let c_opening = open(setup, &c_poly, zeta, c_eval);
    let q_opening = open(setup, &q_poly, zeta, q_eval);

    Proof {
        a_comm,
        b_comm,
        c_comm,
        q_comm,
        a_eval,
        b_eval,
        c_eval,
        q_eval,
        a_opening,
        b_opening,
        c_opening,
        q_opening,
    }
}

pub fn compute_constraint_polynomial(
    a: &DensePolynomial<Fr>,
    b: &DensePolynomial<Fr>,
    c: &DensePolynomial<Fr>,
) -> DensePolynomial<Fr> {
    let ab = a * b;
    &ab - c
}

pub fn divide_by_vanishing(poly: &DensePolynomial<Fr>, root: Fr) -> DensePolynomial<Fr> {
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
