fn main() {
    println!("Project Euler in Rust");

    println!("Problem 1");
    let result: u32 = problem_1();
    println!("Result ~ {:?}", result);

    println!("Problem 2");
    let result: u32 = problem_2();
    println!("Result ~ {:?}", result);
}

fn problem_1() -> u32 {
    const RANGE_MAX: u32 = 1_000;

    let result: u32 = (1..RANGE_MAX).filter(|&x| x % 3 == 0 || x % 5 == 0).sum();
    return result;
}

fn problem_2() -> u32 {
    const MAX_FIB_VAL: u32 = 4_000_000;

    let mut fib_sequence: Vec<u32> = vec![1, 2];
    let mut entry = 0;

    while &entry <= &MAX_FIB_VAL {
        entry = fib_sequence[fib_sequence.len() - 1] + fib_sequence[fib_sequence.len() - 2];
        fib_sequence.push(entry);
    }

    let result: u32 = fib_sequence.iter().filter(|&x| x % 2 == 0).sum();
    return result;
}
