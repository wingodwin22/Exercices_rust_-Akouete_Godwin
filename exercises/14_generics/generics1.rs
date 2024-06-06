// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/* Before correction
fn main() {
    let mut shopping_list: Vec<?> = Vec::new();
    shopping_list.push("milk");
}*/


// After correction

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}

/// Explication
/// ici le fallait juste specifier le type des elements du vecteur.
/// danc notre il s'agit d'une chain de caractere