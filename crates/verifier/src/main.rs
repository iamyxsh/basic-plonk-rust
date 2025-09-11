use circuit::square_circuit::SquareCircuit;
use circuit::witness::Witness;
use proof::prove;
use setup::TrustedSetup;
use verifier::verify;

fn main() {
    let setup = TrustedSetup::generate(10);
    let circuit = SquareCircuit::new();
    let witness = Witness::new(5);

    let proof = prove(&setup, &circuit, &witness);
    let valid = verify(&setup, &circuit, &proof);

    println!("Proof valid: {valid}");
}
