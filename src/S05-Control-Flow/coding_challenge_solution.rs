fn color_to_number_if(color: &str) {
    if color == "red" {
        println!("1");
    } else if color == "green" {
        println!("2");
    } else if color == "blue" {
        println!("3");
    } else {
        println!("0");
    }
}

fn color_to_number_match(color: &str) {
    match color {
        "red" => println!("1"),
        "green" => println!("2"),
        "blue" => println!("3"),
        _ => println!("0"),
    }
}

// Task 3: Factorial Function (without recursion)
fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

// Task 3: Factorial Function (with recursion)
fn factorial_recursive(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

fn main() {
    color_to_number_if("red");
    color_to_number_match("green");

    // Test factorial functions
    println!("Factorial of 5 (iterative): {}", factorial(5));
    println!("Factorial of 4 (iterative): {}", factorial(4));
    println!("Factorial of 5 (recursive): {}", factorial_recursive(5));
    println!("Factorial of 4 (recursive): {}", factorial_recursive(4));
}
