

fn main() {
    use std::collections::VecDeque;

    let a = [1, 2, 3];

    let doubled: Vec<i32> = a.iter()
                             .map(|&x| x * 2)
                             .collect();

    let another_double = a.iter().map(|x| x * 2).collect::<Vec<i32>>();                    
    
    assert_eq!(vec![2, 4, 6], doubled);
    assert_eq!(vec![2, 4, 6], another_double);



    // -----------------------------------


    let b = [1;5];

    println!("b => {b:?}")
}