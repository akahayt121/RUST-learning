use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Game");

    let secret_code=rand::thread_rng().gen_range(1..=100);
    println!("The gentated secret number is {secret_code}");

    loop{
    println!("Guess a number:");
    let mut guess:String = String::new();
    io::stdin().read_line(&mut guess).expect("There is some error");

    // let guess:i32 =guess.trim().parse().expect("Fail to pass"); //same variable name "shadowing"
    
    let guess:i32 =match guess.trim().parse(){
        Ok(num)=>num,
        Err(_) =>{
            println!("Please enter valid input");
            continue;
        },
    };
    println!("The value you guess is :{guess}");

    match guess.cmp(&secret_code){
        Ordering::Less => println!("Value is less"),
        Ordering::Greater => println!("Value is Greater"),
        Ordering::Equal => {
            println!("Values are equal");
            break;
            }
    }
    }
    println!("Thankyou!!!");
    println!("Ok bye");
}
