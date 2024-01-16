// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}


fn main() {

    assert_eq!(
        generate_nametag_text("Beyoncé".into()),
        Ok("Hi! My name is Beyoncé".into())
    );

    assert_eq!(
        generate_nametag_text("".into()),
        // Don't change this line
        Err("`name` was empty; it must be nonempty.".into())
    );

}
