

fn main() {
    let number: u128 = 36;
    println!("Prime factors of {} are \n{:?}", number, prime_factors(number));
    println!("Euler phi of {} is \n{}", number, euler_phi(number));
    
    
}

fn euler_phi(n: u128) -> u128 {
    let mut factors: Vec<u128> = prime_factors(n);
    factors.dedup();

    let mut result: u128 = n;
    for factor in factors {
        result = result * (factor - 1) / factor; 
    }
    return result;
}



fn prime_factors(mut n: u128) -> Vec<u128> {
    let mut p: u128 = 2;
    let mut factors: Vec<u128> = Vec::new();
    
    if n <= 1 {
        return factors; // 0 and 1 are neither prime nor composite
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
    
    return factors;
}