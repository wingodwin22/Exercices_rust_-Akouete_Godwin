// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/*  Before correction
fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num;
}*/

// After correction

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}

// Explication
// ici le probleme est que la fonction square fait bien la multiplication mais ne retourne rien.
// il faut juste retourner le resultat
