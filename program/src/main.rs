//! A simple program to be proven inside the zkVM.
#![no_main]

sp1_zkvm::entrypoint!(main);

pub fn main() {
    println!("parsing the tree...");
    let serialized_tree = sp1_zkvm::io::read_vec();
    let mut buf_view = trivial_tree::buf_view::BufView::wrap(&serialized_tree);
    let db = trivial_tree::Node::<32>::parse(&mut buf_view).unwrap();
    let root_hash = db.hash();

    println!("done: {:x?}.", root_hash);

    sp1_zkvm::io::commit_slice(&root_hash);
}
