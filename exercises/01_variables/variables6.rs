// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// const NUMBER: u32 = 3;
// fn main() {
//     println!("Number {}", NUMBER);

//     let guess: u32 = "42".parse().expect("Not a number!");

//     println!("Another number {}", guess);
// }


// ============================== NEW STUFF ===============================

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

