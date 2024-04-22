pub fn get_sum(string: String) -> u32 {
    let mut sum: u32 = 0;
    let buf = string.as_bytes();
    for i in buf.to_owned() {
        sum = sum + i as u32;
    }

    return sum;
}

pub fn get_raw(string: String) -> Vec<u8> {
    let buf = string.as_bytes().to_owned();

    return buf;
}
