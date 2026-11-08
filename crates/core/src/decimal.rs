pub fn digits(value: u32) -> usize {
    let mut digits = 1;
    let mut value = value;
    while value >= 10 {
        value /= 10;
        digits += 1;
    }
    digits
}

pub fn write(out: &mut Vec<u8>, value: u32) {
    let digits = digits(value);
    let len = out.len();
    out.resize(len + digits, 0);

    let mut value = value;
    let mut index = len + digits;
    while index > len {
        index -= 1;
        out[index] = b'0' + (value % 10) as u8;
        value /= 10;
    }
}
