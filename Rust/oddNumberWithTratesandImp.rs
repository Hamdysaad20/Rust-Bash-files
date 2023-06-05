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
