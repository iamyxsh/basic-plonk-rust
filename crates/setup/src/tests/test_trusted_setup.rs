use crate::*;
use ark_bls12_381::Bls12_381;
use ark_ec::pairing::Pairing;

#[test]
fn test_setup_size() {
    let setup = TrustedSetup::generate(10);
    assert_eq!(setup.g1_powers.len(), 11);
    assert_eq!(setup.max_degree(), 10);
}

#[test]
fn test_first_power_is_generator() {
    let setup = TrustedSetup::generate(5);
    assert_eq!(setup.g1_powers[0], G1Affine::generator());
}

#[test]
fn test_powers_are_consistent() {
    let setup = TrustedSetup::generate(5);

    for i in 0..setup.max_degree() {
        let lhs = Bls12_381::pairing(setup.g1_powers[i], setup.g2_tau);
        let rhs = Bls12_381::pairing(setup.g1_powers[i + 1], setup.g2_gen);
        assert_eq!(lhs, rhs, "Pairing check failed at power {}", i);
    }
}
