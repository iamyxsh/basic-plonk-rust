use circuit::square_circuit::SquareCircuit;
use circuit::witness::Witness;

#[test]
fn test_square_circuit_with_valid_witness() {
    let circuit = SquareCircuit::new();

    // x = 5: 5² = 25 ✓
    let witness = Witness::new(5);
    assert!(circuit.is_satisfied(&witness));
}

#[test]
fn test_square_circuit_with_negative_root() {
    let circuit = SquareCircuit::new();

    // x = -5: (-5)² = 25 ✓
    // In finite field arithmetic, -5 is a large number (p - 5)
    // but (-5) * (-5) = 25 still holds
    let witness = Witness::new_signed(-5);
    assert!(circuit.is_satisfied(&witness));
}

#[test]
fn test_square_circuit_rejects_invalid_witness() {
    let circuit = SquareCircuit::new();

    // x = 6: 6² = 36 ≠ 25 ✗
    let bad_witness = Witness::new(6);
    assert!(!circuit.is_satisfied(&bad_witness));

    // x = 4: 4² = 16 ≠ 25 ✗
    let bad_witness = Witness::new(4);
    assert!(!circuit.is_satisfied(&bad_witness));
}
