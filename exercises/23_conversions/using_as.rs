// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/* Before correction
fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
*/


// After correction
fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}



/// Explication
/// Dans la fonction `average`, j'ai ajouté une conversion explicite du résultat de la division en `f64` en utilisant l'opérateur `as`. 
/// Cette conversion est nécessaire pour garantir que le résultat de la division est du même type que celui attendu par la fonction, c'est-à-dire un `f64`. 
/// Sans cette conversion, le compilateur pourrait rencontrer une ambiguïté sur le type de retour, car la méthode `len` renvoie un entier (`usize`). 
/// En ajoutant `as f64`, nous clarifions explicitement que le résultat doit être traité comme un `f64`, évitant ainsi toute erreur de type et garantissant que le code compile correctement.