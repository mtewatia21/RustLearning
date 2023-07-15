fn main() {
    //1. Variable mutability
    let x = 6; // x is immutable here.
    // x = 8;     // This line will throw an error
    let mut y = 5; // y is a mutable variable
    y = 2;
    println!("The value of x is: {x}.");
    println!("The updated value of y is: {y}.");


    //2. Shadowing

    let z = "Rust";
    let z = 2;              // changed the type of the variable z from being a string to an int.
    println!("Value of z: {z}.");

    let p = 6;
    let p = p+1;
    {
        let p = p*21;
        println!("Value of p in this scope: {p}.")
    }
    println!("Value of p: {p}.");


    //3. Data Types
    let guess:u32 = "42".parse().expect("A!");
    println!("Value of guess: {guess}.");
}
