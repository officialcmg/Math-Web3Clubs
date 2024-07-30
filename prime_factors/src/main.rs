
fn main() {
    let number: u128 = 4;
    match factorize(number) {
        Some(factors) => {
            println!("{} is a composite number with factors: \n{:?}", number, factors);
        }
        None => {
            println!("{} is a prime number.", number);
        }
    }
    
}


fn factorize(mut n: u128) -> Option<Vec<u128>> {
    let mut p: u128 = 2;
    let mut factors: Vec<u128> = Vec::new();
    
    if n <= 1 {
        return None; // 0 and 1 are neither prime nor composite
    }
    while p * p <= n {
        if n % p == 0 {
            factors.push(p);
            n /= p;
        } else {
            p += 1;
        }
    }
    factors.push(n);
    

    if factors.len() == 1 {
        return None; // Only one factor found; number is prime
    } else {
        return Some(factors); // Multiples factors found; number is composite
    }
}


