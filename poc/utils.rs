// XOR in place
pub fn xor_ip(a: &mut Vec<u8>, o: &Vec<u8>) {
    let alen = a.len();
    let olen = o.len();

    let max = if alen > olen {
        alen
    } else if olen > alen {
        olen
    } else {
        olen
    };

    for k in 0..max {
        if k < max {
            a[k] = a[k] ^ o[k];
        } else {
            break;
        }
    }
}
pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.into_iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}

pub fn scrub_with_byte(data: &mut Vec<u8>, byte: u8) {
    let data_range = 0..data.len();
    for n in data_range {
        data[n] = byte;
    }
}
pub fn zerofill(data: &mut Vec<u8>) {
    scrub_with_byte(data, 0);
}

pub fn scrub(data: &mut Vec<u8>) {
    let mut type_range = (0..u8::MAX).into_iter().collect::<Vec<u8>>();
    type_range.reverse();

    zerofill(data);
    for k in type_range {
        scrub_with_byte(data, k);
    }
    zerofill(data);
}

pub fn drop(data: &mut Vec<u8>) {
    let length = data.len();
    let other = (0..length).into_iter().map(|_| 0).collect::<Vec<u8>>();
    xor_ip(data, &other);
    scrub_with_byte(data, 0x7);
    scrub_with_byte(data, 0x0);
    scrub_with_byte(data, 0x1);
    zerofill(data);
    scrub(data);
    zerofill(data);
}
