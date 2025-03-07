// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

/* Before correction
#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
fn fill_vec() -> Vec<i32> {
    // Instead, let's create and fill the Vec in here - how do you do that?
    let mut vec = vec;

    vec.push(88);

    vec
}
*/

// Afetr correction
// ici on a deux possibilitee
// possiblite 1

#[test]
fn main() {
    let vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec() -> Vec<i32> {
    
    let mut vec = vec![22, 44, 66]; 

    vec.push(88); 

    vec 
}

// Possibilite 2 

#[test]
fn main() {
    let vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec() -> Vec<i32> {

    let mut vec = Vec::new();
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.push(88);

    vec
}

// Explication
// dans la premiere possibilite on declare le vecteur initiale dans fill_vec puis on fais vec.push(88) pour ajouter 88 aux elements du veteur.
// dans la second possibilite on cree un vecteur vide puis on fais des push pour ajouter des elements au vecteur.