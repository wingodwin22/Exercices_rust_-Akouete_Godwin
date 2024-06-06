// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

/* Before correction
fn main() {
    let a = ???

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}*/

//After correction
fn main() {
    let a = "je n'ateint pas 100 caracteres";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}

// Explication
// la syntaxe a.len() permet de compter le nomdre de caractere de a et vu a n'avait pas de caratere et que les ?? n'etait pas type, c'est pour cela que le code ne fonctionnait pas.
// pour corriger cela, il faut attribuer une chaine de caractere a "a" ou lui attribuer un tableau de caractere.