pub fn n_digits(mut v: u32) -> usize {
    let mut count = 0usize;
    while v != 0 {
        count += 1;

        v /= 10;
    }

    return count;
}
