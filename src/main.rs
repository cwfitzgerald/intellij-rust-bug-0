use smallvec::{smallvec, SmallVec};

mod works; // Working example

fn main() {
    let mut x: SmallVec<[usize; 3]> = smallvec!(2); // Go to definition fails on SmallVec.
    x.push(3); // Autocomplete/go to definition fails on members
}
