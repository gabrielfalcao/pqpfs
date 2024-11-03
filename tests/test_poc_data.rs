use pqpfs::*;

#[test]
fn test_data_add() {
    assert_eq!(data![1, 2, 3] + data![5, 8, 13], data![6, 10, 16]);
}
#[test]
fn test_data_sub() {
    assert_eq!(data![6, 10, 16] - data![5, 8, 13], data![1, 2, 3]);
}

#[test]
fn test_data_div() {
    assert_eq!(data![20, 30, 60] / data![2, 10, 20], data![10, 3, 3]);
}

#[test]
fn test_data_mul() {
    assert_eq!(data![2, 3, 6] * data![10, 8, 6], data![20, 24, 36]);
}

#[test]
fn test_data_rem() {
    assert_eq!(data![12, 13, 16] % data![10, 8, 6], data![2, 5, 4]);
}
