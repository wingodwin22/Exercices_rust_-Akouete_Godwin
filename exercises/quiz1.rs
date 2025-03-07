// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)

// I AM NOT DONE

/* Before correction
// Put your function here!
// fn calculate_price_of_apples {

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
*/

// After correction
fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity > 40 {
        quantity
    } else {
        quantity * 2
    }
}

#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}


/// Explication
/// J'ai implémenté la fonction `calculate_price_of_apples` qui prend en paramètre la quantité de pommes achetées et retourne le prix total. 
/// Dans la fonction, j'utilise une instruction `if` pour vérifier si la quantité est supérieure à 40. 
/// Si c'est le cas, le prix total est simplement égal à la quantité, sinon le prix est égal à la quantité multipliée par 2. 
/// Les tests vérifient que la fonction retourne les prix corrects pour différentes quantités de pommes.