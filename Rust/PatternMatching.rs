//use pattern matching to play with data :)
fn main() {
     let tuple:(i8,i8,i8) = (1, 2, 3);
     //assign tuple values to variables
     let (a, b, c) = tuple;
     println!("a = {}, b = {}, c = {}", a, b, c);

     //use is as a function parameter
        print_tuple(tuple);
 }

    fn print_tuple((first_val,second_val,third_val): (i8, i8, i8)) {
        println!("printed as values \n {},{}", second_val, third_val);
    }
