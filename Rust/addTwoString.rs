fn main() {
    let mut owned_string: String = "hello ".to_owned();
    let another_owned_string: String = "world".to_owned();
    
    owned_string.push_str(&another_owned_string);
    println!("{}", owned_string);
}
