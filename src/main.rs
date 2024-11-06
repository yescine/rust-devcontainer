use ferris_says::say;
use std::io::{stdout, BufWriter};

fn hello (){
  print!("hello World \n");
  let stdout = stdout();
  let message = String::from("Hello fellow rustaceans!");
  let width = message.chars().count();

  let mut writer = BufWriter::new(stdout.lock());
  say(message.as_bytes(), width, &mut writer).unwrap();
}

fn main () {
  hello();

  let (resp,..) = ("data",4,5,10); 
  let [..,z] = [1,2,3,4,5,6];

  let mut x:i32;
  x = 1;
  x +=4;
  assert_eq!(x,5);

  println!("\nsuccess! {}  with array {}", resp, z);

}