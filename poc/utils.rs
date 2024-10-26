pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.into_iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}

pub fn scrub_with_byte(data: &mut [u8], byte: u8) {
    let data_range = 0..data.len();
    for n in data_range {
        data[n] = byte;
    }
}
pub fn zerofill(data: &mut [u8]) {
    scrub_with_byte(data, 0);
}

pub fn scrub(data: &mut [u8]) {
    let mut type_range = (0..u8::MAX).into_iter().collect::<Vec<u8>>();
    type_range.reverse();

    zerofill(data);
    for k in type_range {
        scrub_with_byte(data, k);
    }
    zerofill(data);
}
