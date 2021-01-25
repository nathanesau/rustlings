// move_semantics_additional.rs
// Make me compile! Execute `rustlings hint move_semenatics_additional` for hints :)

// run using ``rustlings run move_semantics_additional.rs``

fn main() {
    int_example1();
    str_example1();
    str_example2();
    str_example3();
}

fn int_example1() {
    
    // create int, ``x`` is owner
    let mut x = 5;

    {
        // create reference to x,  ``x`` is still owner
        let xr = &mut x;

        println!("xr is {}", xr);

        // modify mutable reference, which modifies x
        *xr += 1;

        println!("xr is {}", xr);
    }

    println!("x is {}", x);
}

fn str_example1() {
    fn hello(a: String) -> String {
        println!("Inside {}", a);
        a
    }

    // create string, ``s`` is owner
    let mut s = String::from("string");

    // pass s to hello, ``s`` is no long owner
    // however, we can return the new owner
    s = hello(s);
    println!("{}", s);
}

fn str_example2() {
    fn hello(a: &String) {
        println!("Inside {}", a);
    }

    // create string, ``s`` is owner
    let mut s = String::from("string");

    // pass const reference to s to hello, ``s`` is still owner
    hello(&s);
    println!("{}", s);
}

fn str_example3() {
    fn hello(a: &mut String) {
        println!("Inside {}", a);
        a.push_str("suffix");
    }

    // create string: ``s`` is owner
    let mut s = String::from("string");

    // pass reference to s to hello, ``s`` is still owner
    hello(&mut s);
    println!("{}", s);
}