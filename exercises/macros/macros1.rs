// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// run using ``rustlings run macros1``

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
