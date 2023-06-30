use reduce;
use std::vec::Vec;

#[test]
fn test_empty_slice() {

    let empty_vec = Vec::new();

    let sum = reduce::parallel_foldl(
        &empty_vec,
        0,
        |a, b| a + b,
        |a, b| a + b
    );

    assert_eq!(sum, 0);
}

#[test]
fn test_single_element_slice() {

    let single_element_vec = vec![1; 1];

    let sum = reduce::parallel_foldl(
        &single_element_vec,
        0,
        |a, b| a + b,
        |a, b| a + b
    );

    assert_eq!(sum, 1);

}