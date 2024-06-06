// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
/* Before correction
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = ???

    assert_eq!([2, 3, 4], nice_slice)
}*/

// After correction
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}

// Explication 
// dans un tableau, les elements sont indexe de 0-n donc pour que notre code passe,
// il faut extraire la partie du tableau correspondant a [2, 3, 4].
// donc avec la syntax &a[1..4] on a pointe les elements du tableau correspondant a ces indexes sachant que l'indexe 4 n'est pas inclus.