// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE


//before correction
//fn main() {
//    call_me(3);
//}

//fn call_me(num: ) {
//    for i in 0..num {
//       println!("Ring! Call number {}", i + 1);
//    }
//}

// After correction

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

// Explication
// ici la fonction a ete declare sans le type de l'argument "num"

