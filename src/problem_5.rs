pub fn run() -> u64 {
    let range = 1..=20;
    let mut value = 0;

    'outer: loop {
        value += 1;

        'inner: for x in range.clone() {
            if value % x != 0 { continue 'outer; }
        }
        return value;
    }
}
