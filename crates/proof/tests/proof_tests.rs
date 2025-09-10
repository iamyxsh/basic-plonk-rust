use ark_bls12_381::Fr;
use ark_ff::{Field, One, Zero};
use ark_poly::{DenseUVPolynomial, Polynomial, univariate::DensePolynomial};
use circuit::{square_circuit::SquareCircuit, witness::Witness};
use proof::{compute_constraint_polynomial, divide_by_vanishing, prove};
use setup::TrustedSetup;

#[test]
fn test_constraint_polynomial_is_zero_at_gate() {
    let circuit = SquareCircuit::new();
    let witness = Witness::new(5);
    let wires = circuit.wire_values(&witness);

    let a_poly = DensePolynomial::from_coefficients_vec(vec![wires.a]);
    let b_poly = DensePolynomial::from_coefficients_vec(vec![wires.b]);
    let c_poly = DensePolynomial::from_coefficients_vec(vec![wires.c]);

    let t_poly = compute_constraint_polynomial(&a_poly, &b_poly, &c_poly);

    let t_at_one = t_poly.evaluate(&Fr::one());
    assert!(t_at_one.is_zero());
}

#[test]
fn test_constraint_polynomial_nonzero_for_bad_witness() {
    let circuit = SquareCircuit::new();
    let witness = Witness::new(6);
    let wires = circuit.wire_values(&witness);

    let a_poly = DensePolynomial::from_coefficients_vec(vec![wires.a]);
    let b_poly = DensePolynomial::from_coefficients_vec(vec![wires.b]);
    let c_poly = DensePolynomial::from_coefficients_vec(vec![wires.c]);

    let t_poly = compute_constraint_polynomial(&a_poly, &b_poly, &c_poly);

    let t_at_one = t_poly.evaluate(&Fr::one());
    assert!(!t_at_one.is_zero());
    assert_eq!(t_at_one, Fr::from(11u64));
}

#[test]
fn test_quotient_polynomial() {
    let circuit = SquareCircuit::new();
    let witness = Witness::new(5);
    let wires = circuit.wire_values(&witness);

    let a_poly = DensePolynomial::from_coefficients_vec(vec![wires.a]);
    let b_poly = DensePolynomial::from_coefficients_vec(vec![wires.b]);
    let c_poly = DensePolynomial::from_coefficients_vec(vec![wires.c]);

    let t_poly = compute_constraint_polynomial(&a_poly, &b_poly, &c_poly);
    let q_poly = divide_by_vanishing(&t_poly, Fr::one());

    let x = Fr::from(7u64);
    let t_at_x = t_poly.evaluate(&x);
    let q_at_x = q_poly.evaluate(&x);
    let vanishing_at_x = x - Fr::one();

    assert_eq!(t_at_x, q_at_x * vanishing_at_x);
}

#[test]
fn test_prove_creates_valid_structure() {
    let setup = TrustedSetup::generate(10);
    let circuit = SquareCircuit::new();
    let witness = Witness::new(5);

    let proof = prove(&setup, &circuit, &witness);

    assert_eq!(proof.a_eval, Fr::from(5u64));
    assert_eq!(proof.b_eval, Fr::from(5u64));
    assert_eq!(proof.c_eval, Fr::from(25u64));
}

#[test]
fn test_prove_with_negative_root() {
    let setup = TrustedSetup::generate(10);
    let circuit = SquareCircuit::new();
    let witness = Witness::new_signed(-5);

    let proof = prove(&setup, &circuit, &witness);

    let neg_five = -Fr::from(5u64);
    assert_eq!(proof.a_eval, neg_five);
    assert_eq!(proof.b_eval, neg_five);
    assert_eq!(proof.c_eval, Fr::from(25u64));
}
