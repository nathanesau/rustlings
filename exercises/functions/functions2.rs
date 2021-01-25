// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// run using ``rustlings run functions2``

fn main() {
    call_me(3);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
