# PLONK Proof System

A from-scratch implementation of the PLONK zero-knowledge proof system in Rust.

**Circuit:** Proves knowledge of `x` such that `x² = 25` without revealing `x`.

## Architecture

```
┌─────────┐     ┌─────────┐     ┌─────────┐
│  setup  │────▶│   kzg   │────▶│ prover  │
└─────────┘     └─────────┘     └────┬────┘
                     │               │
                     ▼               ▼
               ┌─────────┐     ┌──────────┐
               │ circuit │────▶│ verifier │
               └─────────┘     └──────────┘
```

| Crate | Purpose |
|-------|---------|
| `setup` | Trusted setup — generates powers of τ |
| `kzg` | Polynomial commitments using BLS12-381 |
| `circuit` | Gate constraints and witness definition |
| `prover` | Proof generation |
| `verifier` | Proof verification |

## The Protocol

**Prover** (knows secret `x = 5`):
1. Encodes wire values as polynomials: `a(X) = x`, `b(X) = x`, `c(X) = 25`
2. Commits to wire polynomials using KZG
3. Computes constraint polynomial: `t(X) = a(X)·b(X) - c(X)`
4. Computes quotient: `q(X) = t(X) / (X - 1)`
5. Commits to quotient
6. Evaluates all polynomials at challenge point ζ
7. Creates KZG opening proofs for each evaluation

**Verifier** (knows only that output is 25):
1. Verifies all KZG opening proofs
2. Checks constraint: `a(ζ)·b(ζ) - c(ζ) = q(ζ)·(ζ - 1)`
3. Checks public input: `c(ζ) = 25`

## PLONK Gate

The arithmetic gate equation:

```
qL·a + qR·b + qO·c + qM·(a·b) + qC = 0
```

For multiplication (`a × b = c`): `qL=0, qR=0, qO=-1, qM=1, qC=0`

## What's Implemented

- KZG polynomial commitment scheme (commit, open, verify)
- BLS12-381 pairing operations via arkworks
- Single multiplication gate
- Quotient polynomial argument
- Complete proof generation and verification

## What's Simplified

| Component | This Implementation | Production PLONK |
|-----------|---------------------|------------------|
| Challenge ζ | Hardcoded | Fiat-Shamir hash |
| Gates | 1 | Thousands |
| Permutation | None | Copy constraints |
| Blinding | None | Random polynomials |
| Setup | Single τ | MPC ceremony |

## Usage

```bash
cargo test --workspace   # Run all tests
cargo run                # Run demo
```

## Output

```
Proof valid: true
```

The verifier accepts proofs for `x = 5` and `x = -5` (both satisfy `x² = 25`).
The verifier rejects proofs for any other value.

## Dependencies

- `ark-bls12-381` — BLS12-381 curve
- `ark-poly` — Polynomial operations
- `ark-ec` — Elliptic curve traits
- `ark-ff` — Finite field arithmetic
