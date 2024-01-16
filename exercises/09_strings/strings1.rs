// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);


    let hello = String::from("Здравствуйте");

    let answer = &hello[0..2];

    println!("[0] of {hello} is {answer}");
}

fn current_favorite_color() -> String {
    String::from("blue")
}
