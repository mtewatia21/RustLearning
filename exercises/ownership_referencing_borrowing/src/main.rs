fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

                                    let s1 = gives_ownership();         // gives_ownership moves its return
                                    // value into s1

let s2 = String::from("hello");     // s2 comes into scope

let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                    // takes_and_gives_back, which also
                                    // moves its return value into s3

// Below demostrates that I can keep on using variable str, if I pass it as a reference to a function.
// I still retain the ownership in that scope.
let mut str = String::from("Ni Hao, Buddy!");
let lref = calculate_length_referencing(&str);
println!("Length of String - {str} : {lref}");
append_to_string(&mut str);
println!("New String -> {str}");

let (str2, l) = calculate_length(str);
println!("Length of String - {str2} : {l}");



} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens. Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}


// Referencing - This is important since taking ownership and then giving it back for every 
// simple task invloving a variable that is passed to a function and to be used after the 
// function call would make it very tedious. This is demostrated below:

// Ownership Transfer method to calculate the length of a string.
fn calculate_length(s: String) -> (String, usize){
    let l = s.len();
    (s, l)
}

// Referencing method to calculate the length of the string.
fn calculate_length_referencing(s: &String) -> (usize){
    let l = s.len();
    l
}


// Mutable References
fn append_to_string(s: &mut String){
    s.push_str("string");
}


// Dangling References - Rust ensures that we will not have dangling references.

// dangle function will not compile. Comment below function to compile this code.
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}