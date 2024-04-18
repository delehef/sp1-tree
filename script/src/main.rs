//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use trivial_tree::Node;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn prove_tree(t: &[u8]) {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    stdin.write_slice(t);
    let client = ProverClient::new();
    let proof = client.prove(ELF, stdin).expect("proving failed");

    // // Read output.
    // let found = &proof.public_values.buffer.data;
    // println!("found:        {:x?}", found);

    // Verify proof.
    client.verify(ELF, &proof).expect("verification failed");

    // proof
    //     .save("proof-with-io.json")
    //     .expect("saving proof failed");
}

fn main() {
    let left_tree = Node::<32>::random_tree(5, 10).serialize();
    let right_tree = Node::<32>::random_tree(5, 10).serialize();
    prove_tree(&left_tree);
    prove_tree(&right_tree);
}
