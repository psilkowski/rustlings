// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// I AM NOT DONE

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {

        // step 1.
        let num_of_chars = s.chars().count();
        if 0 == num_of_chars {
            return Err(Self::Err::Empty);
        }

        // step 2.
        let words: Vec<&str> = s.split(',').collect();

        if words.len() != 2 {
            return Err(Self::Err::BadLen);
        }

        // step 3.
        let temp_name = words[0];
        let temp_age = words[1];

        if temp_name.is_empty() {
            return Err(Self::Err::NoName);
        }

        let age_check = temp_age.parse::<usize>();
        let age: usize;

        match age_check {
            Ok(x) => age = x,
            Err(e) => return Err(Self::Err::ParseInt(e))
        }

        Ok(Person { name: String::from(temp_name), age: age })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);

    assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    
    let p = "John,32".parse::<Person>();
    assert!(p.is_ok());

    let p = p.unwrap();
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 32);

    assert!(matches!(
        "John,".parse::<Person>(),
        Err(ParsePersonError::ParseInt(_))
    ));

    assert!(matches!(
        "John,twenty".parse::<Person>(),
        Err(ParsePersonError::ParseInt(_))
    ));

    assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));

    assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));

    assert!(matches!(
        ",".parse::<Person>(),
        Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
    ));

    assert!(matches!(
        ",one".parse::<Person>(),
        Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
    ));

    assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));

    assert_eq!(
        "John,32,man".parse::<Person>(),
        Err(ParsePersonError::BadLen)
    );
}

