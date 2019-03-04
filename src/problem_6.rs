pub fn run() -> u64 {
    return square_of_sum(100) - sum_of_squares(100);
}

pub fn sum_of_squares(n: u64) -> u64 {
    return (1..=n).fold(0, |acc, x| acc + x.pow(2));
}

pub fn square_of_sum(n: u64) -> u64 {
    return (1..=n).sum::<u64>().pow(2);
}
