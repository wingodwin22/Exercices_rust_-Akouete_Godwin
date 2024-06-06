// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/* Before correction
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!();
    }

    #[test]
    fn is_false_when_odd() {
        assert!();
    }
}
*/


// After correction
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2), true);
    }

    #[test]
    fn is_false_when_odd() {
        assert!(is_even(7), false);
    }
}

/// Explication
/// ici on doit utiliser les fonction du test pour verifier si les nombre sont reellement paire ou pas.
/// donc j'ai passe une valeur 2 is_even que je fais comparer avec true vu que is_even retourne un boolean pour verifier is_true_when_even
/// et j'ai passe une valeur 7 is_even que je fais comparer avec false vu que is_even pour verifier is_false_when_odd