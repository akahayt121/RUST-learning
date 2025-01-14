use std::io;
fn main(){
    // step 1: the variable in which the value been assigned by user should be mutable using mut
    // step 2: when we take a variable to assign a value then the mutable refrence should go under readline() argument
    // step 3: to handle the error a method is called "expect("error occured")" and crash the program
    let mut user_value=String::new();

    println!("Please enter a value:");
    
    io::stdin().read_line(&mut user_value).expect("There is some error in program");

    println!("The value input is: {user_value}");
}