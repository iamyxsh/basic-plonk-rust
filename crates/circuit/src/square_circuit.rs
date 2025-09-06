use ark_bls12_381::Fr;

use crate::gate::Gate;
use crate::witness::{WireValues, Witness};

#[derive(Clone, Debug)]
pub struct SquareCircuit {
    pub gate: Gate,
    pub public_output: Fr,
}

impl SquareCircuit {
    /// Create the x² = 25 circuit
    pub fn new() -> Self {
        SquareCircuit {
            gate: Gate::multiplication(),
            public_output: Fr::from(25u64),
        }
    }

    pub fn wire_values(&self, witness: &Witness) -> WireValues {
        WireValues {
            a: witness.x,
            b: witness.x,
            c: self.public_output,
        }
    }

    pub fn is_satisfied(&self, witness: &Witness) -> bool {
        let wires = self.wire_values(witness);
        self.gate.is_satisfied(wires.a, wires.b, wires.c)
    }
}

impl Default for SquareCircuit {
    fn default() -> Self {
        Self::new()
    }
}
