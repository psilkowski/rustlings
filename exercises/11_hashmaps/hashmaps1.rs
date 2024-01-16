// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    basket.insert(String::from("cherry"), 1);

    basket.insert(String::from("orange"), 2);

    basket
}


fn main() {
    

    let basket1 = fruit_basket();
    assert!(basket1.len() >= 3);


    let basket2 = fruit_basket();
    assert!(basket2.values().sum::<u32>() >= 5);

}
