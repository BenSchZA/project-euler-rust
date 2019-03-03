pub fn run() -> u64 {
    const RANGE_MAX: u64 = 1_000;

    let result: u64 = (1..RANGE_MAX).filter(|&x| x % 3 == 0 || x % 5 == 0).sum();
    return result;
}