pub fn run() -> u64 {
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