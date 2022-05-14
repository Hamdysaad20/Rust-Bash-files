use std::io;
//take a user input and print it
fn main() {
println!("put a number :");
    let mut input :String = String::new();
    io::stdin().read_line(&mut input).expect("Erorr !");
    println!("the number you put is : {} ",input);
}
