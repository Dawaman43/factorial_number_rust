    use std::io;

    fn main() {
        println!("Factorial sequence:");
        println!("Enter a number to calculate its factorial value:");
        let mut input = String::new();
        io::stdin().
            read_line(&mut input)
            .expect("Failed to read line");
        
        let input: u128 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                return;
            }
        };

        let mut mul: u128 = 1;
        
        for n in 0..input {
            mul *= input - n;
        }

        println!("mul is {mul}")
    }
