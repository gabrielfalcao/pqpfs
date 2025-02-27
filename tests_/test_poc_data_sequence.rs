use pqpfs::*;

#[test]
fn test_data_vec_iter() {
    let mut seq = DataSeq::new();
    seq.push(data![1, 2, 3]);
    seq.push(data![5, 8, 13]);
    seq.push(data![6, 10, 16]);
    let mut iter = seq.iter();
    assert_eq!(iter.next(), Some(data![1, 2, 3]));
    assert_eq!(iter.next(), Some(data![5, 8, 13]));
    assert_eq!(iter.next(), Some(data![6, 10, 16]));
    assert_eq!(iter.next(), None);
}


#[test]
fn test_data_vec_to_data_from_data() {
    let mut seq = DataSeq::new();
    seq.push(data![1, 2, 3]);
    seq.push(data![5, 8, 13]);
    seq.push(data![6, 10, 16]);
    let data = seq.to_data().expect("Data");
    assert_eq!(data, data![0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x08, 0x0d, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x0a, 0x10, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(seq, DataSeq::from_data(&data).expect("DataSeq"))
}
