pub fn run() -> u64 {
    let range: Vec<_> = (100*100..999*999)
        .collect::<Vec<_>>()
        .iter().cloned()
        .filter(|&x| is_palindrome(number_to_vec(x)) && check_factors_three_digits(x))
        .collect::<Vec<_>>();
    return *range.last().unwrap();
}

fn is_palindrome(v: Vec<u64>) -> bool {
    v.iter().eq(v.iter().rev())
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

fn check_factors_three_digits(val: u64) -> bool {
    const MIN_FAC: u64 = 100;
    const MAX_FAC: u64 = 999;

    for x in MIN_FAC..=MAX_FAC {
        for y in MIN_FAC..=MAX_FAC {
            if x*y == val {
                return true
            }
        }
    }
    return false
}