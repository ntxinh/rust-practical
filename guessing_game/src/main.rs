use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change
        // The :: syntax in the ::new line indicates that new is an associated function of the String type
        let mut guess = String::new(); // mutable

        // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        // If you don’t call expect, the program will compile, but you’ll get a warning
        // Rust warns that you haven’t used the Result value returned from read_line, indicating that the program hasn’t handled a possible error.
        // The right way to suppress the warning is to actually write error-handling code, but in our case we just want to crash this program when a problem occurs, so we can use expect.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow the previous value of `guess` with a new one
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        // The same to:
        // println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
