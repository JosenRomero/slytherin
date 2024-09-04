use std::io::stdin; // the input/output functionality from the standard library

pub mod utils;

#[tokio::main] // Tokio is an event-driven, non-blocking I/O platform for writing asynchronous applications
async fn main() {

  println!("Welcome! \n");

  println!("Press any key to get a random coffee image");
  
  let mut input: String = String::new();
  
  /* 
  read_line method: handle to get input from the user.
    The '&myVariable' indicates that this argument is a reference. references are immutable by default.
    then '&mut myVariable' to make it mutable.
  */
  stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  let res: Result<String, reqwest::Error> = utils::get_random_coffee_img().await;

  match res {
    Ok(img) => println!("{img}"),
    Err(_) => println!("Error")
  }

}
