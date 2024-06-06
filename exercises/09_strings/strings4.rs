// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM NOT DONE

/* Before correction
fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    ???("blue");
    ???("red".to_string());
    ???(String::from("hi"));
    ???("rust is fun!".to_owned());
    ???("nice weather".into());
    ???(format!("Interpolation {}", "Station"));
    ???(&String::from("abc")[0..1]);
    ???("  hello there ".trim());
    ???("Happy Monday!".to_string().replace("Mon", "Tues"));
    ???("mY sHiFt KeY iS sTiCkY".to_lowercase());
}*/


// After correction 
fn string_slice(arg: &str) {
    println!("{}", arg);
}

fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // &str
    string("red".to_string()); // String
    string(String::from("hi")); // String
    string("rust is fun!".to_owned()); // String
    string("nice weather".into()); // String
    string(format!("Interpolation {}", "Station")); // String
    string_slice(&String::from("abc")[0..1]); // &str
    string_slice("  hello there ".trim()); // &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // String
}

// Explication
//"blue" : Littéral de chaîne, type &str, donc on utilise string_slice.
//"red".to_string(); "rust is fun!".to_owned() sont Conversion en String, donc on utilise string.
//String::from("hi") : Création d'un String, donc on utilise string.
//"nice weather".into() : Conversion implicite en String, donc on utilise string.
//format!("Interpolation {}", "Station") : Renvoie un String, donc on utilise string.
//&String::from("abc")[0..1] : Tranche de String, type &str, donc on utilise string_slice.
//" hello there ".trim() : Méthode trim renvoie un &str, donc on utilise string_slice.
//"Happy Monday!".to_string().replace("Mon", "Tues") : replace sur un String renvoie un String, donc on utilise string.
//"mY sHiFt KeY iS sTiCkY".to_lowercase() : to_lowercase renvoie un String, donc on utilise string.