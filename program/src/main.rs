//! A simple program to be proven inside the zkVM.
#![no_main]

use buf_view::BufView;

sp1_zkvm::entrypoint!(main);

mod tree;
type EWord = [u8; 32];

pub fn main() {
    println!("parsing the tree...");
    let serialized_tree = sp1_zkvm::io::read_vec();
    let mut buf_view = BufView::wrap(&serialized_tree);
    let db = tree::Node::parse(&mut buf_view).unwrap();
    db.pretty();
    println!("done");

    println!("computing root hash...");
    let root_hash = db.hash();

    println!("done: {:x?}.", root_hash);

    sp1_zkvm::io::commit_slice(&root_hash);
}
