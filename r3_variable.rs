// the variable in rust is by default immutable variable

fn main(){
    let age=23;
    // age=10;    //this will throw an error as varibles are immutable
    println!("My age is : {age}");

    // if we want a variable to be mutable then we have to declear it as mutable using mute keyward

    let mut car="Maruti";
    println!("The name of the Car is {car}");

    car="BMW";
    println!("The name of the Car is {car}");
}