use reduce;


fn main() {

    let my_vec = vec![1; 100];

    let sum = reduce::parallel_foldl(
        &my_vec,
        0,
        |a, b| { a + b },
        |a, b| { a + b }
    );

    println!("Sum: {}", sum);
}