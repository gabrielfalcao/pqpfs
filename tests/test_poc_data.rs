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
