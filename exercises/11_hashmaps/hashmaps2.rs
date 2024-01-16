// hashmaps2.rs
//
// We're collecting different fruits to bake a delicious fruit cake. For this,
// we have a basket, which we'll represent in the form of a hash map. The key
// represents the name of each fruit we collect and the value represents how
// many of that particular fruit we have collected. Three types of fruits -
// Apple (4), Mango (2) and Lychee (5) are already in the basket hash map. You
// must add fruit to the basket so that there is at least one of each kind and
// more than 11 in total - we have a lot of mouths to feed. You are not allowed
// to insert any more of these fruits!
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: Insert new fruits if they are not already present in the
        // basket. Note that you are not allowed to put any type of fruit that's
        // already present!

        basket.entry(fruit).or_insert(2);
    }
}

// Don't modify this function!
fn get_fruit_basket() -> HashMap<Fruit, u32> {
    let mut basket = HashMap::<Fruit, u32>::new();
    basket.insert(Fruit::Apple, 4);
    basket.insert(Fruit::Mango, 2);
    basket.insert(Fruit::Lychee, 5);
    basket
}

fn main() {

    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
    assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
    assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);

    let mut basket1 = get_fruit_basket();
    fruit_basket(&mut basket1);
    let count_fruit_kinds = basket1.len();
    assert!(count_fruit_kinds >= 5);

    let mut basket2 = get_fruit_basket();
    fruit_basket(&mut basket2);
    let count = basket2.values().sum::<u32>();
    assert!(count > 11);
    
    let mut basket3 = get_fruit_basket();
    fruit_basket(&mut basket3);
    for amount in basket3.values() {
        assert_ne!(amount, &0);
    }

    for (key, val) in basket3.iter() {
        println!("key: {:#?} val: {val}", key);
    }

}