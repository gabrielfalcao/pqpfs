use pqpfs::Data;

#[test]
fn test_data_add() {
    assert_eq!(
        Data::from(vec![1u8, 2u8, 3u8]) + Data::from(vec![5u8, 8u8, 13u8]),
        Data::from(vec![6u8, 10u8, 16u8])
    );
}
#[test]
fn test_data_sub() {
    assert_eq!(
        Data::from(vec![6u8, 10u8, 16u8]) - Data::from(vec![5u8, 8u8, 13u8]),
        Data::from(vec![1u8, 2u8, 3u8])
    );
}

#[test]
fn test_data_div() {
    assert_eq!(
        Data::from(vec![20u8, 30u8, 60u8]) / Data::from(vec![2u8, 10u8, 20u8]),
        Data::from(vec![10u8, 3u8, 3u8])
    );
}


#[test]
fn test_data_mul() {
    assert_eq!(
        Data::from(vec![2u8, 3u8, 6u8]) * Data::from(vec![10u8, 8u8, 6u8]),
        Data::from(vec![20u8, 24u8, 36u8])
    );
}
