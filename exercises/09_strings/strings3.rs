// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // let mut temp_str = String::from(input);

    // for c in input.chars() {
    //     if c == ' ' {
    //         temp_str.chars().next();

    //         println!("Found heading whitespace");
    //     }
    //     else
    //     {
    //         break;
    //     }
    // }

    // println!("step1 = {temp_str}");

    // for c in input.chars().rev() {
    //     if c == ' ' {
    //         temp_str.chars().next_back();

    //         println!("Found trailing whitespace");
    //     }
    //     else
    //     {
    //         break;
    //     }
    // }

    // println!("step2 = {temp_str}");
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!

    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // let temp_str = String::from(input);

    // temp_str.replace("cars", "balloons")

    input.replace("cars", "balloons")
}

fn main() {
    // trim_me("Hello!     ");
    // trim_me("  What's up!");
    // trim_me("   Hola!  ");


    assert_eq!(trim_me("Hello!     "), "Hello!");
    assert_eq!(trim_me("  What's up!"), "What's up!");
    assert_eq!(trim_me("   Hola!  "), "Hola!");

    assert_eq!(compose_me("Hello"), "Hello world!");
    assert_eq!(compose_me("Goodbye"), "Goodbye world!");


    assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
    assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
}
