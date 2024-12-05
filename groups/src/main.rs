fn main() {
    let element1 = 3;
    let element2 = 4;
    let modulus1 = 5;
    let modulus2 = 7;

    match order_of_direct_product(element1, element2, modulus1, modulus2) {
        Some(order) => println!("The order of the element in the direct product group is: {}", order),
        None => println!("Could not determine the order of the element."),
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

fn order_of_element(element: i32, modulus: i32) -> Option<i32> {
    let mut result = 1;
    let mut current = element % modulus;

    while current != 1 {
        current = (current * element) % modulus;
        result += 1;

        if result > modulus { // Prevent infinite loops
            return None;
        }
    }
    Some(result)
}

fn order_of_direct_product(element1: i32, element2: i32, modulus1: i32, modulus2: i32) -> Option<i32> {
    let order1 = order_of_element(element1, modulus1)?;
    let order2 = order_of_element(element2, modulus2)?;
    Some(lcm(order1, order2))
}