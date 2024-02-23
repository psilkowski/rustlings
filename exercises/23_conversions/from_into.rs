// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results

// I AM NOT DONE

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        
        // step 1.
        let num_of_chars = s.chars().count();
        if 0 == num_of_chars {
            return Person::default();
        }

        // step 2.
        let words: Vec<&str> = s.split(',').collect();

        if words.len() < 2 {
            return Person::default();
        }

        // step 3.
        let temp_name = words[0];
        let temp_age = words[1];

        if temp_name.is_empty() {
            return Person::default();
        }

        let age_check = temp_age.parse::<usize>();
        let age: usize;

        match age_check {
            Ok(x) => age = x,
            _ => return Person::default()
        }

        Person { name: String::from(temp_name), age: age }

        // match s.split_once(',') {
        //     Some((first, second)) => {
        //         if first.is_empty() {
        //             Person::default()
        //         }
        //         else if let Ok(a) = second.parse::<usize>()
        //         {
        //             Person {name: first.into(), age: a}
        //         }
        //         else
        //         {
        //             Person::default()
        //         }
        //     },
        //     _ => Person::default()

        // } // end of match
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);

    // Test that the default person is 30 year old John
    let dp = Person::default();
    assert_eq!(dp.name, "John");
    assert_eq!(dp.age, 30);

    // Test that John is returned when bad string is provided
    let p = Person::from("");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);

    // Test that "Mark,20" works
    let p = Person::from("Mark,20");
    assert_eq!(p.name, "Mark");
    assert_eq!(p.age, 20);

    // Test that "Mark,twenty" will return the default person due to an
    // error in parsing age
    let p = Person::from("Mark,twenty");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);

    let p: Person = Person::from("Mark");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);

    let p: Person = Person::from("Mark,");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);

    let p: Person = Person::from(",1");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);

    let p: Person = Person::from(",");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);

    let p: Person = Person::from(",one");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);

    let p: Person = Person::from("Mike,32,");
    assert_eq!(p.name, "Mike");
    assert_eq!(p.age, 32);

    let p: Person = Person::from("Mike,32,man");
    assert_eq!(p.name, "Mike");
    assert_eq!(p.age, 32);
}


