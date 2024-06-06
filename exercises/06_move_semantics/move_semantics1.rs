// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

/* Before correction
#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;

    vec.push(88);

    vec
}
*/

// After correction

#[test]
fn main() {
    let mut vec0 = vec![22, 44, 66];

    fill_vec(&mut vec0);

    assert_eq!(vec0, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}

// Explication
// ici le souci est que le code quand la fonction fill_vec prend possession de vec0 ce qui rend vec0 vide.
// ce qui fait qu'on perd la valeur initial du vecteur.
// pour resoudre ce probleme, j'ai modifie fill_vec pour qu'elle prenne une reference au vecteur initial au lieu de prendre possession du vecteur.