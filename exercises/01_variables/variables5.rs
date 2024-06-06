// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// Before correction
//fn main() {
//    let number = "T-H-R-E-E"; // don't change this line
//    println!("Spell a Number : {}", number);
//    number = 3; // don't rename this variable
//    println!("Number plus two is : {}", number + 2);
//}


// After correction
fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number: i32 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}


// Explication
// la variable "number" etant declaree comme un string, ne pouvais prendre ensuite une valeur de type string.
// donc il falait redefinire le type de number