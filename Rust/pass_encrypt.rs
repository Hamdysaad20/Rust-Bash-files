// [dependencies]
// magic-crypt ="3.1.9"

use magic_crypt::{ MagicCryptTrait, new_magic_crypt};

fn main() {
 let pass:String = input();
  let hashed:String = encrypt(pass);
  print!("hashed pass : {}",hashed);

// let normal_pass_again = decrypt(hashed);

}

fn input() -> String{
  use std::io::{stdin,stdout,Write};
  let mut s=String::new();
  print!("Please enter some text: ");
  let _=stdout().flush();
  stdin().read_line(&mut s).expect("Did not enter a correct string");
  if let Some('\n')=s.chars().next_back() {
    s.pop();
  }
  if let Some('\r')=s.chars().next_back() {
    s.pop();
  }
  return s;
}
fn encrypt(pass:String) -> String{
  let mcrypt = new_magic_crypt!("magick", 256); //Creates an instance of the magic crypt library/crate.
  let encrypted_string = mcrypt.encrypt_str_to_base64(pass);
  return encrypted_string;
}
fn  decrypt(encrypted_string:String) ->String{
  let mcrypt = new_magic_crypt!("magick", 256); //Creates an instance of the magic crypt library/crate.

  let decrypted_string = mcrypt.decrypt_base64_to_string(&encrypted_string).unwrap();
  return decrypted_string;
}
