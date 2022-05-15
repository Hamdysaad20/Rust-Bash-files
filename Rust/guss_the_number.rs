use std::cmp::Ordering;
use std::io;
use rand::Rng;

//create gussing number game
//get the number of tries
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries_number:i32 =0;
    loop {

        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        tries_number = tries_number + 1;

        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Too small! \n number of tries: {}", tries_number),
            Ordering::Greater => println!("Too big! \n number of tries: {}", tries_number),
            Ordering::Equal => {
                println!("You win! \n number of tries: {}", tries_number);
                break;
            }
        }
    }
}
