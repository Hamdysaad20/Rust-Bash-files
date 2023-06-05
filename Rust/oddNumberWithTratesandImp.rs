// var
trait Printable {
   fn print(&self);
}

impl Printable for i32 {
   fn print(&self) {

       for x in 0..*self {
           if x % 2 != 0 {
               println!("Odd: {}", x);
           }
       }
   }
}

fn main(){

  // implentation of Printable trait for i32
    let x: i32 = 100;
    x.print();

}
---
//vector
trait Printable {
   fn print(&self);
}

impl Printable for Vec<i32> {
   fn print(&self) {
       self.iter().for_each(|x| println!("{}", x));
   }
}

fn main(){

    let x = vec![1,2,3,4,5];
    x.print();

}
