// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("L00000000NG ONE");
    let string2 = String::from("xyz");

    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    
    println!("The longest string is '{}'", result);
}


// 1) static?
// 2) move println! to lesser scope
// 3) move string2 to bigger scope