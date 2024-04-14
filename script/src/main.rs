//! A simple script to generate and verify the proof of a given program.

use std::io::BufWriter;

use rand::distributions::{Alphanumeric, DistString};
use sp1_sdk::{SP1Prover, SP1Stdin, SP1Verifier};
use tree::Node;

mod tree;

const CONTRACTS_PER_BLOCK: usize = 20;
const VAR_PER_CONTRACT: usize = 120;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
type EWord = [u8; 32];

fn eword(x: &str) -> EWord {
    let mut bs = x.as_bytes().to_vec();
    assert!(bs.len() <= 32);
    bs.resize(32, 0u8);
    bs.try_into().unwrap()
}

fn a(x: &str) -> [u8; 20] {
    let mut bs = x.as_bytes().to_vec();
    assert!(bs.len() <= 20);
    bs.resize(20, 0u8);
    bs.try_into().unwrap()
}

fn strand(l: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), l)
}

fn make_var() -> Node {
    Node::Variable {
        name: strand(8),
        value: eword(&strand(32)),
    }
}

fn make_contract() -> Node {
    Node::Contract {
        address: a(&strand(20)),
        storage: (0..VAR_PER_CONTRACT).map(|_| make_var()).collect(),
    }
}

fn main() {
    let db = Node::Block {
        number: 125,
        contracts: (0..CONTRACTS_PER_BLOCK).map(|_| make_contract()).collect(),
    };
    println!("The DB:");
    db.pretty();
    let mut out = BufWriter::new(Vec::new());
    db.serialize(&mut out);
    let serialized = out.into_inner().unwrap();

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    stdin.write_slice(&serialized);
    let proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let found = &proof.public_values.buffer.data;
    println!("expected:     {:x?}", db.hash());
    println!("found:        {:x?}", found);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
