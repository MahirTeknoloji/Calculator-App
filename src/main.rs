
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add(a, b) => a + b,
            Operation::Subtract(a, b) => a - b,
            Operation::Multiply(a, b) => a * b,
            Operation::Divide(a, b) => a / b,
        }
    }
}

fn main() {

    println!("Calculator");
    println!("-----------");
    println!("Operations: ");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("-----------");

    println!("Enter the first number: ");
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = a.trim().parse().expect("Please type a number!");

    println!("Enter the second number: ");
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = b.trim().parse().expect("Please type a number!");

    println!("Enter the operation: ");
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();

    let operation = match operation {
        "1" => Operation::Add(a, b),
        "2" => Operation::Subtract(a, b),
        "3" => Operation::Multiply(a, b),
        "4" => Operation::Divide(a, b),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result = operation.calculate();
    println!("Result: {}", result);



}
