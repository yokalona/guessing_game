use ctrlc;
use std::io;
use rand::Rng;
use std::io::Write;
use std::str::FromStr;
use term_cursor;

enum Guess { Ok(u32), Err }

fn header() {
  print!("{}", term_cursor::Clear);
  println!(r#"
    ___                       _   _                                  _                _
   / _ \_   _  ___  ___ ___  | |_| |__   ___   _ __  _   _ _ __ ___ | |__   ___ _ __ / \
  / /_\/ | | |/ _ \/ __/ __| | __| '_ \ / _ \ | '_ \| | | | '_ ` _ \| '_ \ / _ \ '__/  /
 / /_\\| |_| |  __/\__ \__ \ | |_| | | |  __/ | | | | |_| | | | | | | |_) |  __/ | /\_/
 \____/ \__,_|\___||___/___/  \__|_| |_|\___| |_| |_|\__,_|_| |_| |_|_.__/ \___|_| \/

 "#);
}

fn footer() {
  println!(r#"
                                            _
  _   _  ___  _   _  __      _____  _ __   / \
 | | | |/ _ \| | | | \ \ /\ / / _ \| '_ \ /  /
 | |_| | (_) | |_| |  \ V  V / (_) | | | /\_/
  \__, |\___/ \__,_|   \_/\_/ \___/|_| |_\/
  |___/

 "#)
}

fn welcome() {
  print!(" Please, enter your guess: ");
}

fn next_round() {
  welcome();
  io::stdout().flush().unwrap();
}

fn generate_number(boundary: u32) -> u32 {
  let number = rand::thread_rng().gen_range(1..=boundary);
  print!(" I thought of a number, can You guess it?\n\n");
  number
}

fn read_input() -> Result<u32, <u32 as FromStr>::Err> {
  let mut guess = String::new();
  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
  guess.trim().parse()
}

fn get_boundary() -> u32 {
  loop {
    print!(" Let's set maximum value for the number I'll think of: ");
    io::stdout().flush().unwrap();
    match read_input() {
      Ok(boundary) if boundary > 1 => return boundary,
      Ok(_) | Err(_)               => {
        println!(" The value should be bigger than 1 and smaller than: {}", u32::MAX);
      }
    }
  }
}

fn get_guess(tries: &mut u32, boundary: u32) -> Guess {
  match read_input() {
    Ok(guess) if withing_boundaries(guess, boundary) => {
      *tries += 1;
      Guess::Ok(guess)
    },
    Ok(guess)                                        => {
      message(guess, format!("Sorry, {guess} is not between 1 and {boundary}"));
      Guess::Err
    }
    Err(_)                                           => {
      println!(" The guess could be only the number between 1 and {boundary}.");
      Guess::Err
    },
  }
}

fn withing_boundaries(num: u32, boundary: u32) -> bool {
  num >= 1 && num <= boundary
}

fn number_length(number: u32) -> u32 {
  number.checked_ilog10().unwrap_or(0) + 1
}

fn message(guess: u32, message: String) {
  print!("{}", term_cursor::Up(1));
  welcome();
  let intent = (0..number_length(u32::MAX) - number_length(guess)).map(|_| " ").collect::<String>();
  println!("{guess}.{intent} {message}.");
}

fn too_small(guess: u32) {
  message(guess, format!("Sorry, {guess} is too small"));
}

fn too_large(guess: u32) {
  message(guess, format!("Sorry, {guess} is too big"));
}

fn you_won(answer: u32, tries: u32) {
  message(answer, format!("{answer} is the correct answer, congratulations!"));
  footer();
  print!("\n It took You {tries} tries to guess correctly");
  if tries > 1 {
    println!(", but can you do better?\n");
  } else {
    println!("\n");
  }
}

fn on_give_up(riddle: u32) {
  ctrlc::set_handler(move || {
    println!("\n\n {riddle} was the correct answer\n");
    std::process::exit(0)
  })
  .expect("Error setting Ctrl-C handler")
}

fn main() {
  header();
  let mut tries = 0;
  let boundary = get_boundary();
  let riddle = generate_number(boundary);
  on_give_up(riddle);
  loop {
    next_round();
    match get_guess(&mut tries, boundary) {
      Guess::Ok(guess) if guess < riddle => too_small(guess),
      Guess::Ok(guess) if guess > riddle => too_large(guess),
      Guess::Ok(_)                       => break you_won(riddle, tries),
      Guess::Err                         => continue,
    };
  }
}
