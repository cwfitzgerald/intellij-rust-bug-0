use smallvec::SmallVec; // Don't bring macro into scope

fn works() {
    let x: SmallVec<[usize; 8]> = SmallVec::new(); // Autocomplete/go to definition works as expected.
}

fn works2() {
    use smallvec::smallvec; // Local includes also work for some reason.
    let mut x: SmallVec<[usize; 3]> = smallvec!(2); // SmallVec stuff still works
    x.push(3); // Autocomplete still works
}
