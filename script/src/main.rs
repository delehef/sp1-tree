//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{SP1Prover, SP1Stdin, SP1Verifier};
use trivial_tree;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let serialized = trivial_tree::random_tree(4, 4);

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    stdin.write_slice(&serialized);
    let proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let found = &proof.public_values.buffer.data;
    // println!("expected:     {:x?}", db.hash());
    println!("found:        {:x?}", found);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
