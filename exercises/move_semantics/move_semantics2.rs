// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// run using ``rustlings run move_semantics2``

fn main() {
    let mut vec0 = Vec::new();

    // we don't want to modify vec0 here!
    //
    // therefore approach should be 
    // (1) pass vec0 by reference
    // (2) clone vec0 inside fill_vec
    // (3) push to clone of vec0
    // (4) return modified clone (which will be moved) 
    //
    // also vec1 should be mutable since we need to modify it after println!
    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut filled_vec = vec.clone();

    filled_vec.push(22);
    filled_vec.push(44);
    filled_vec.push(66);

    filled_vec
}
