use crate::*;
use ark_ff::One;

/// poly(&[1, 2, 3]) = 1 + 2x + 3x²
fn poly(coeffs: &[i64]) -> DensePolynomial<Fr> {
    let fr_coeffs: Vec<Fr> = coeffs
        .iter()
        .map(|&c| {
            if c >= 0 {
                Fr::from(c as u64)
            } else {
                -Fr::from((-c) as u64)
            }
        })
        .collect();
    DensePolynomial::from_coefficients_vec(fr_coeffs)
}

#[test]
fn test_commit_and_verify_opening() {
    let setup = TrustedSetup::generate(10);

    // f(x) = 1 + 2x + 3x²
    let f = poly(&[1, 2, 3]);

    let commitment = commit(&setup, &f);

    // f(5) = 1 + 2(5) + 3(25) = 1 + 10 + 75 = 86
    let z = Fr::from(5u64);
    let y = f.evaluate(&z);
    assert_eq!(y, Fr::from(86u64));

    let proof = open(&setup, &f, z, y);

    assert!(verify(&setup, &commitment, z, y, &proof));
}

#[test]
fn test_verify_rejects_wrong_evaluation() {
    let setup = TrustedSetup::generate(10);
    let f = poly(&[1, 2, 3]);

    let commitment = commit(&setup, &f);

    let z = Fr::from(5u64);
    let correct_y = f.evaluate(&z);
    let wrong_y = Fr::from(87u64);

    let proof = open(&setup, &f, z, correct_y);

    assert!(verify(&setup, &commitment, z, correct_y, &proof));
    assert!(!verify(&setup, &commitment, z, wrong_y, &proof));
}

#[test]
fn test_verify_rejects_wrong_proof() {
    let setup = TrustedSetup::generate(10);

    let f = poly(&[1, 2, 3]);
    let g = poly(&[5, 6, 7]);

    let commitment_f = commit(&setup, &f);

    let z = Fr::from(5u64);
    let y_f = f.evaluate(&z);

    let wrong_proof = open(&setup, &g, z, g.evaluate(&z));

    assert!(!verify(&setup, &commitment_f, z, y_f, &wrong_proof));
}

#[test]
fn test_constant_polynomial() {
    let setup = TrustedSetup::generate(10);

    let f = poly(&[42]);

    let commitment = commit(&setup, &f);

    let z = Fr::from(999u64);
    let y = Fr::from(42u64);

    let proof = open(&setup, &f, z, y);

    assert!(verify(&setup, &commitment, z, y, &proof));
}

#[test]
fn test_linear_polynomial() {
    let setup = TrustedSetup::generate(10);

    // f(x) = 3 + 7x
    let f = poly(&[3, 7]);

    let commitment = commit(&setup, &f);

    // f(10) = 3 + 70 = 73
    let z = Fr::from(10u64);
    let y = f.evaluate(&z);
    assert_eq!(y, Fr::from(73u64));

    let proof = open(&setup, &f, z, y);

    assert!(verify(&setup, &commitment, z, y, &proof));
}

#[test]
fn test_divide_by_linear() {
    // (x² - 1) / (x - 1) = x + 1
    let numerator = poly(&[-1, 0, 1]);
    let root = Fr::one();

    let quotient = divide_by_linear(&numerator, root);

    let expected = poly(&[1, 1]);
    assert_eq!(quotient.coeffs(), expected.coeffs());
}
