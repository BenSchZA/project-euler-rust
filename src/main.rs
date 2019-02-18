fn main() {
    println!("Project Euler in Rust");

    let current_problem = 4;
    let result = -1;

    match current_problem {
        1 => {let result: u32 = problem_1();}
        2 => {let result: u32 = problem_2();}
        3 => {let result: u64 = problem_3();}
        4 => {let result: u64 = problem_4();}
    }
    println!("Problem {}", current_problem);
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

fn problem_3() -> u64 {
    const NUM: u64 = 600_851_475_143;
    let sqrt_num: u64 = (NUM as f64).sqrt() as u64;

    let mut factors: Vec<u64> = Vec::new();
    let mut factor = 1;

    println!("> Calculating factors");
    while &factor <= &sqrt_num {
        if &NUM%&factor == 0 {
            factors.push(factor);
            if &factor != &(NUM/&factor) {
                factors.push(&NUM/&factor);
            }
        }
        factor = &factor + 1;
    }

    println!("> Finding biggest prime factor");
    for val in factors.iter().rev() {

        println!("{}", val);
        if is_prime(*val) {
            return *val;
        }
    }

    panic!("No prime factors");
}

fn problem_4() -> u64 {
    let result = (100*100..999*999).filter(|&x| is_palindrome(number_to_vec(x))).collect::<Vec<_>>().sort().last();
    return result;
}

fn is_palindrome(n: Vec<u64>) -> bool {
    println!("{:?}", n);
    if n.len() % 2 == 0 {
        let half_len = n.len()/2;
        let half_vec = n.chunks(half_len);
        if half_vec[0] == half_vec[1] {
            return true;
        }
    } else {
        return false;
    }
    return false;
}

fn number_to_vec(n: u64) -> Vec<u64> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    return digits
}

fn is_prime(val: u64) -> bool {
    // See http://en.wikipedia.org/wiki/AKS_primality_test
    match val {
        2 => return true,
        3 => return true,
        _ => {
            if val % 2 == 0 { return false }
            if val % 3 == 0 { return false }

            let mut i: u64 = 5;
            let mut w: u64 = 2;

            while i * i <= val {
                if val % i == 0 { return false }
                i = i + w;
                w = 6 - w;
            }
            return true
        }
    }
}