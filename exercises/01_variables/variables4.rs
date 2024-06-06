// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

//Before correction

//fn main() {
//    let x = 3;
//    println!("Number {}", x);
//    x = 5; // don't change this line
//    println!("Number {}", x);
//}

// After correction

fn main() {
    let mut x: i32 = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}

//explication 
// on ne peut pas assigner deux valeurs a une variable immutable.
// donc il faut rendre x mutable avec la syntax "mut"