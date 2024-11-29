use pqpfs::*;

#[test]
fn test_data_seq_iter() {
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
