use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..=1000);
    loop{
    println!("Enter a number please!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Bad Input!");

    let guess: u32 = match guess.trim().parse(){   // shadowing a variable, allows a var name to be reused.
        Ok(num) => num,
        Err(_) => continue,
    };  
    
    println!("Secret Number is: {secret_number}");

    println!("Your guess is: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => {
            println!("Yay! You win!");
            break;
        }
    }
}
}
