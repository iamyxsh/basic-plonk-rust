use ark_bls12_381::Fr;
use ark_ff::One;
use circuit::square_circuit::SquareCircuit;
use kzg::verify as kzg_verify;
use proof::Proof;
use setup::TrustedSetup;

pub fn verify(
    setup: &TrustedSetup,
    circuit: &SquareCircuit,
    proof: &Proof,
) -> bool {
    let zeta = Fr::from(7u64);

    if !kzg_verify(setup, &proof.a_comm, zeta, proof.a_eval, &proof.a_opening) {
        return false;
    }

    if !kzg_verify(setup, &proof.b_comm, zeta, proof.b_eval, &proof.b_opening) {
        return false;
    }

    if !kzg_verify(setup, &proof.c_comm, zeta, proof.c_eval, &proof.c_opening) {
        return false;
    }

    if !kzg_verify(setup, &proof.q_comm, zeta, proof.q_eval, &proof.q_opening) {
        return false;
    }

    let vanishing_eval = zeta - Fr::one();

    let constraint_eval = proof.a_eval * proof.b_eval - proof.c_eval;
    let quotient_check = proof.q_eval * vanishing_eval;

    if constraint_eval != quotient_check {
        return false;
    }

    if proof.c_eval != circuit.public_output {
        return false;
    }

    true
}
