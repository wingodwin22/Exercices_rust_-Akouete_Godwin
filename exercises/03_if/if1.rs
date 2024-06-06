// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

/*Before correction
pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
}*/

//After correction
fn main() {
    println!("le plus grand nombre est: {}", bigger(5, 10)); 
    println!("le plus grand nombre est: {}", bigger(15, 10)); 
    println!("a et b sont egalent a : {}", bigger(10, 10)); 
}

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a >= b {
        a
    } else {
        b
    }
}

// explication 
// la fonction bigger est incomplete et la fonction bigger n'a ete appele nul part
// avec la contidion if on compare a et b.
// pour a > ou = a, on retourne a.
// pour b > a, on retourne b
// avec la fonction main, on a appele bigger en lui passant aussi des arguments




// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
