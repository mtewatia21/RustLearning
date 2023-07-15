use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game!");
    let range = rand::thread_rng().gen_range(25..=10000);
    let secret_number = rand::thread_rng().gen_range(1..=range);
    let mut count = 0;
    loop{
        count += 1;
    println!("Enter a number please!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Bad Input!");

    let guess: u32 = match guess.trim().parse(){   // shadowing a variable, allows a var name to be reused.
        Ok(num) => num,
        Err(_) => continue,
    };  

    //println!("Your guess is: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => {
            println!("Yay! You win!");
            println!("Secret Number is: {secret_number}");
            println!("Number of attempts: {count}");
            break;
        }
    }
}
}
