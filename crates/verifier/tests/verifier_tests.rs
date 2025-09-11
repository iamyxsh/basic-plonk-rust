use ark_bls12_381::Fr;
use circuit::square_circuit::SquareCircuit;
use circuit::witness::Witness;
use proof::prove;
use setup::TrustedSetup;
use verifier::verify;

#[test]
fn test_valid_proof_verifies() {
    let setup = TrustedSetup::generate(10);
    let circuit = SquareCircuit::new();
    let witness = Witness::new(5);

    let proof = prove(&setup, &circuit, &witness);
    assert!(verify(&setup, &circuit, &proof));
}

#[test]
fn test_valid_proof_negative_root() {
    let setup = TrustedSetup::generate(10);
    let circuit = SquareCircuit::new();
    let witness = Witness::new_signed(-5);

    let proof = prove(&setup, &circuit, &witness);
    assert!(verify(&setup, &circuit, &proof));
}

#[test]
fn test_invalid_witness_fails_verification() {
    let setup = TrustedSetup::generate(10);
    let circuit = SquareCircuit::new();
    let witness = Witness::new(6);

    let proof = prove(&setup, &circuit, &witness);
    assert!(!verify(&setup, &circuit, &proof));
}

#[test]
fn test_tampered_evaluation_fails() {
    let setup = TrustedSetup::generate(10);
    let circuit = SquareCircuit::new();
    let witness = Witness::new(5);

    let mut proof = prove(&setup, &circuit, &witness);
    proof.a_eval = Fr::from(999u64);

    assert!(!verify(&setup, &circuit, &proof));
}

#[test]
fn test_tampered_public_output_fails() {
    let setup = TrustedSetup::generate(10);
    let circuit = SquareCircuit::new();
    let witness = Witness::new(5);

    let proof = prove(&setup, &circuit, &witness);

    let wrong_circuit = SquareCircuit {
        gate: circuit.gate.clone(),
        public_output: Fr::from(100u64),
    };

    assert!(!verify(&setup, &wrong_circuit, &proof));
}
