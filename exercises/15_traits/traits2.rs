// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut s = Vec::from(self);
        s.push(String::from("Bar"));
        s
    }
}

// TODO: Implement trait `AppendBar` for a vector of strings.

fn main() {

    let mut foo = vec![String::from("Foo")].append_bar();

    println!("{:?}", foo);

    assert_eq!(foo.pop().unwrap(), String::from("Bar"));
    assert_eq!(foo.pop().unwrap(), String::from("Foo"));

}