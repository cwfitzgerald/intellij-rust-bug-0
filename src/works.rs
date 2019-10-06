use smallvec::SmallVec; // Don't bring macro into scope

fn works() {
    let x: SmallVec<[usize; 8]> = SmallVec::new(); // Autocomplete/go to definition works as expected.
}
