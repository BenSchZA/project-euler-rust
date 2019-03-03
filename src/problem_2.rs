pub fn run() -> u64 {
    const MAX_FIB_VAL: u64 = 4_000_000;

    let mut fib_sequence: Vec<u64> = vec![1, 2];
    let mut entry = 0;

    while &entry <= &MAX_FIB_VAL {
        entry = fib_sequence[fib_sequence.len() - 1] + fib_sequence[fib_sequence.len() - 2];
        fib_sequence.push(entry);
    }

    let result: u64 = fib_sequence.iter().filter(|&x| x % 2 == 0).sum();
    return result;
}