use ark_bls12_381::Fr;
use circuit::square_circuit::SquareCircuit;
use circuit::witness::Witness;

#[test]
fn test_wire_values() {
    let circuit = SquareCircuit::new();
    let witness = Witness::new(5);

    let wires = circuit.wire_values(&witness);

    assert_eq!(wires.a, Fr::from(5u64));  // left input = x
    assert_eq!(wires.b, Fr::from(5u64));  // right input = x
    assert_eq!(wires.c, Fr::from(25u64)); // output = 25
}
