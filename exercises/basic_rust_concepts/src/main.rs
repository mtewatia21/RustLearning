use std::io;
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

    // Compund Types -> Tuples and Arrays
    // This program first creates a tuple and binds it to the variable tup. 
    // It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. This is called destructuring because it breaks the single tuple into three parts. Finally, the program prints the value of y, which is 6.4.
    let tup = (500, 6.4, 1);

    let (xt, yt, zt) = tup;

    println!("The value of y is: {yt}");

    let t: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = t.0;

    let six_point_four = t.1;

    let one = t.2;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // Functions
    function2();
}

fn function2(){
    println!("Function2 at work!");
}
