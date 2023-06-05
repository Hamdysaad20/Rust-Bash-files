# use pattern matching to play with data :)

```rs
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

```
---
# extracting a val from a struct and use it 
      NOTE: you can use it in low level and it is useful with functions and if statment level 
```rs
struct MyStruct {
    x: usize,
    y: usize,
    z: usize,
}

fn main() {

    let foo = MyStruct {
        x: 69,
        y: 420,
        z: 1337,
    };

    let MyStruct { x, .. } = foo; // here

    //print the struct
    println!("x: {}", x);
}

```
---
