struct VB{
    a:i32,
    b:i32,
    c:i32
}
fn main(){
    let structure = VB{a:9, b:8, c:7};
    let value=&structure;
    println!("The values are: {} {} {}",value.a,value.b,value.c)
}