use std::vec::Vec;

use reduce;

fn main() {

    let my_vec = Vec::from_iter(1..1);

    let sum = reduce::parallel_foldl(
        &my_vec,
        0,
        |a, b| { a + b },
        |a, b| { a + b }
    );

    println!("Sum: {}", sum);
}