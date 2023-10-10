use std::io::{self, Write};

fn main() {
  let mut keyboard_input = String::new();

  loop {

    print!("Type \"Exit\" to close the game: ");
    io::stdout().flush().unwrap();

    keyboard_input.clear();
    io::stdin().read_line(&mut keyboard_input).unwrap();

    if keyboard_input.eq("Exit\n") {
      println!("Exiting game  :)");
      break;
    }
    else {
      println!("{keyboard_input}");
    }
  }
}
