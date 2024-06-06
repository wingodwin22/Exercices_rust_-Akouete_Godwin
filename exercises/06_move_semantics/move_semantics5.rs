// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

/* Before correction
#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
*/

// After correction
#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}

// Explication
// le code affiche une erreur car on ne peut pas avoir plusieur reference mutable sur une meme variable
// donc au niveau de y et z, apres la modification, la referenciation est leve.