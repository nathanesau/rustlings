// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

// run using ``rustlings run modules1``

mod sausage_factory {
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
