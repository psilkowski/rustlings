// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

fn main() {

    let test = animal_habitat("gopher");
    if test == "Burrow" {
        println!("test1 complete");
    }
    else {
        println!("test1 failed");
    }
    
    let test = animal_habitat("snake");
    if test == "Desert" {
        println!("test2 complete");
    }
    else {
        println!("test2 failed");
    }
    
    let test = animal_habitat("crab");
    if test == "Beach" {
        println!("test3 complete");
    }
    else {
        println!("test3 failed");
    }

    let test = animal_habitat("dinosaur");
    if test == "Unknown" {
        println!("test4 complete");
    }
    else {
        println!("test4 failed");
    }
}
