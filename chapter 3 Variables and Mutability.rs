
// variables in rust is immutable by default
fn main(){
    // let keyword is used to declare variable
    let a=12;
    
    println!("value is{a}");

    // by using mut keyword we can make a variable mutable and value may varie
    let mut b=13;
    println!("value is{b}");

    // by using const keyword value is constant and cannot be changed
    // and a const variable cannot run without defining the type to it
    // const is independent of any scope
    // constant value should not be assigned on runtime (cannot use non constant value)
    const C:u32=14;
    println!("value is{C}");



    //shadowing

    // shadowing is a concept in rust where a variable is re-decleared with same name and different value assigned

    let apple =30;
    println!("The cost of apple is {apple}");

    //the previous value has been shadowed with a new one
    let apple =60;
    println!("The cost of apple is {apple}");

    // now the value of previous apple has be first calculated in expression then assigned to the new variable
    let apple = apple+10;
    println!("The cost of apple is {apple}");

}