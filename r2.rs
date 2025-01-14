/*fn main(){
    let f=100.243201;

    //let i:i32=f; //this will show an error as it is implicit typecasting
    let x:i32 = f as i32;

    println!("The value of i is: {}",x);
    print_type_of(&x);
}

fn print_type_of<T>(_: &T){
    println!("{}",std::any::type_name::<T>())
}

    */

//typecasting in rust only supports explicit typecasting using "as" keyword


fn main(){
    let val='c';

    let change:i32= val as i32;
    println!("The value of 'c' is typecaseted to : {}",change);

    print_type(&change);
}
fn print_type<T>(_: &T){
    println!("{}",std::any::type_name::<T>());
}