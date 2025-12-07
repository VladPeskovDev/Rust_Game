#[derive(Debug)]
enum Operation {
    Add,      
    Subtract, 
    Multiply, 
    Divide,   
}

fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err(String::from("Ошибка: деление на ноль!"))
            } else {
                Ok(a / b)
            }
        }
    }
}

fn main() {
   
    let a = 10.0;
    let b = 5.0;
    

    let operation = Operation::Add;  // Add, Subtract, Multiply, Divide
    
    
    match calculate(a, b, operation) {
        Ok(result) => println!("{} = {}", "Результат", result),
        Err(error) => println!("{}", error),
    }
}