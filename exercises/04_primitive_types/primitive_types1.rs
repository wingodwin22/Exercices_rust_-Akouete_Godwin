// primitive_types1.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)

// I AM NOT DONE

/* Before correction
fn main() {
    // Booleans (`bool`)

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let // Finish the rest of this line like the example! Or make it be false!
    if is_evening {
        println!("Good evening!");
    }
}*/

// after correction

fn main() {
    // Booleans (`bool`)

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = !is_morning;// Finish the rest of this line like the example! Or make it be false!
    if is_evening {
        println!("Good evening!");
    }
}

// Explication
// pour clompeter le code, on declare la varible is_evening avant de le mettre dans la condition.
// j'ai initialise is_evening a is_morning = false avec la syntaxe is_evening = !is_morning
