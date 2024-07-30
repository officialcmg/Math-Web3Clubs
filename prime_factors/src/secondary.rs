
fn main() {
    // let number = 756780;
    let number=12;
    match factorize(number) {
        Some(factors) => {
            println!("{} is a composite number with factors: \n {:?}", number, factors);
        }
        None => {
            println!("{} is a prime number.", number);
        }
    }
    
}


fn factorize(n: u64) -> Option<Vec<u64>> {
    if n <= 1 {
        return None; // 0 and 1 are neither prime nor composite
    }

    let mut factors: Vec<u64> = Vec::new();
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 && is_prime(i) {
                factors.push(i);
        }
    }

    if factors.is_empty() {
        return None; // No factors found, number is prime
    } else {
        return Some(factors); // Factors found, number is composite
    }
}


fn is_prime(n: u64) -> bool {
    let mut flag: bool = true;
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0{
            flag = false;
        }
    }
    return flag;
}