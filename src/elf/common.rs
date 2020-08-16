pub fn get_flag_char<T: Into<u64>>(flags: T, value: T, sign: char) -> char {
    let flags = flags.into() as usize;
    let value = value.into() as usize;

    if flags & value == value {
        sign
    } else {
        ' '
    }
}
