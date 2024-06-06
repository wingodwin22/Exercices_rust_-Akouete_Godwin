// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// Before correction
//fn main() {
//    let x;
//    if x == 10 {
//        println!("x is ten!");
//    } else {
//        println!("x is not ten!");
//    }
//}



// After correction

fn main() {
    let x: i32 = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}


// explication

// la variable x etait declare mais pas initilisee et n'avait aucune valeur