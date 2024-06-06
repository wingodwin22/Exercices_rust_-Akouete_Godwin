// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

/* Before correction

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
*/

// After correction
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(&vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(88);

    vec
}

// Explication
// ici l'erreur est du au transfert des valeur vec0 vers fill_vec ce qui rend vec0 vide.
// pour resoudre cela, au lieu que fill_vec prenne possession de vec0, on fait en sorte que fill_vec prenne refference sur vec0 sans en prendre possession.