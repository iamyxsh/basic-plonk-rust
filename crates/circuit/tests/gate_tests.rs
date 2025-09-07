use ark_bls12_381::Fr;
use circuit::gate::Gate;

#[test]
fn test_multiplication_gate() {
    let gate = Gate::multiplication();

    // 5 * 5 = 25 ✓
    assert!(gate.is_satisfied(
        Fr::from(5u64),
        Fr::from(5u64),
        Fr::from(25u64)
    ));

    // 5 * 5 ≠ 26 ✗
    assert!(!gate.is_satisfied(
        Fr::from(5u64),
        Fr::from(5u64),
        Fr::from(26u64)
    ));

    // 3 * 7 = 21 ✓
    assert!(gate.is_satisfied(
        Fr::from(3u64),
        Fr::from(7u64),
        Fr::from(21u64)
    ));
}

#[test]
fn test_addition_gate() {
    let gate = Gate::addition();

    // 3 + 7 = 10 ✓
    assert!(gate.is_satisfied(
        Fr::from(3u64),
        Fr::from(7u64),
        Fr::from(10u64)
    ));

    // 3 + 7 ≠ 11 ✗
    assert!(!gate.is_satisfied(
        Fr::from(3u64),
        Fr::from(7u64),
        Fr::from(11u64)
    ));
}
