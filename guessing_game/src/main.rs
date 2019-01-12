use std::io;  // input-output library from std
use std::cmp::Ordering;  // match comparison
use rand::Rng;  // random number generation

fn main() {

    println!("Guess the number!");
    // generate number in current thread using operating systen.
    // lower inclusive, upper exclusive
    let secret_number = rand::thread_rng().gen_range(1, 11);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // new string object

        // pass ref of string object to stdin().read_line
        match io::stdin().read_line(&mut guess) {
            Ok(str) => str,
            Err(_) => {
                println!("Failed to read line, try again.");
                continue;
            }
        };
            //.expect("Failed to read line.");

        // trim white space (leading and trailing spaces, and the \n from the enter press)
        // parse the string in to the annotated data type (u32, unsigned 32bit integer)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,  // success. value is held by returned Ok enum
            Err(_) => {  // _ because we don't care WHAT the error was right now.
                println!("That wasn't a number.");
                continue;
            }
        };

        println!("You guessed {}", guess);

        // cmp returns an enum called Ordering, which will take on diff values from the results
        // of the match. Similar to the Result enum which returns Ok or Err variants.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Correct!");
                break;  // end loop
            },
        };
    }
}
