fn function2(){
    println!("Function2!!");
}
fn main() {
    println!("Hello, world!");
    function2();
    function3(21);
    function4(21, 'm');

    let x = give_me21();
    println!("21 is given -> {x}");

    let y = factorial(21);
    println!("Factorial is: {y}");

    basic_if(10);
    is_div(21);
    if_in_let_stmt();
}

fn function3(x: i32){
    println!("The value of x: {x}");
}

fn function4(val: i32, unit: char){
    println!("The value is: {val}{unit}");
}

// expressions do not end in a semi-colon i.e. ;
// only statements do because they do not evaluate to anything.
fn give_me21() -> i32{
    21
} 

// A function to calculate a factorial in RUST
fn factorial(x: i128) -> i128{
    if x <= 1 {
        return 1;
    }
    x*factorial(x-1)
}

// control flow

// basic if
fn basic_if(x: i32){
    if x < 21{
        println!("Number is less than 21. Its value is: {x}");
    }
}

fn is_div(number: i32){
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let_stmt(){
    let condition = true;
    let number = if condition {21} else {6};
    println!("Value is: {number}")
}