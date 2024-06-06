// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/* Before correction
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
*/

// After correction
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

// Explication
// depuis la fonction main on ne pouvait pas appeler la fonction make_sausage car elle est dans une autre module et n'est pas directement connu de main.
// le rendre visible pour main j'ai mis pub pour le rendre public et j'ai laisse get_secret_recipe intact pour ne pas le rendre publique.