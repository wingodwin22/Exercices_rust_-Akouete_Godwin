// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/* Before correction

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 scoops left. At 10PM, someone eats it
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0. The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, 5);
    }
}
*/

// After correction
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    if time_of_day > 23 {
        None
    } else if time_of_day >= 22 {
        Some(0)
    } else {
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // On peut utiliser un pattern match pour extraire la valeur de l'Option
        let icecreams = maybe_icecream(12);
        match icecreams {
            Some(value) => assert_eq!(value, 5),
            None => panic!("No ice cream available"),
        }
    }
}


/// Explication
/// j'ai mis des conditions avec if et else.
/// Si time_of_day est supérieur à 23, la fonction retourne None.
/// Si time_of_day est 22 ou plus, la fonction retourne Some(0) (car il n'y a plus de glace).
/// Sinon, la fonction retourne Some(5) (car il reste de la glace).
/// pour tester les conditions, j'ai Utilise match pour extraire la valeur de l'Option. 
/// Si l'Option contient une valeur, la valeur est comparée à 5. 
/// Si l'Option est None, le test panique avec un message d'erreur.