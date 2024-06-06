// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
/* Before correction
fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue"
}
*/

// After correction

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string
}

// explication
// ici la fonction devrait retourner un string mais il retourne une chaine de caractere literal.
// pour le corriger, on convertie la valeur en string.
