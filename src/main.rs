use std::io;
use rand::Rng;
use std::io::Write;
use std::str::FromStr;

enum Guess { Ok(u32), Err }
const BOUNDARY: u32 = 100;

fn header() {
  println!(r#"
    ___                       _   _                                  _                _
   / _ \_   _  ___  ___ ___  | |_| |__   ___   _ __  _   _ _ __ ___ | |__   ___ _ __ / \
  / /_\/ | | |/ _ \/ __/ __| | __| '_ \ / _ \ | '_ \| | | | '_ ` _ \| '_ \ / _ \ '__/  /
 / /_\\| |_| |  __/\__ \__ \ | |_| | | |  __/ | | | | |_| | | | | | | |_) |  __/ | /\_/
 \____/ \__,_|\___||___/___/  \__|_| |_|\___| |_| |_|\__,_|_| |_| |_|_.__/ \___|_| \/

 "#);
  print!("\t");
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

fn next_round(tries: &mut u32) {
  print!("\tPlease, enter your guess: ");
  io::stdout().flush().unwrap();
  *tries += 1;
}

fn generate_number() -> u32 {
  rand::thread_rng().gen_range(1..=BOUNDARY)
}

fn read_input() -> Result<u32, <u32 as FromStr>::Err> {
  let mut guess = String::new();
  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
  guess.trim().parse()
}

fn get_guess() -> Guess {
  match read_input() {
    Ok(num) if num > 1 && num < BOUNDARY => Guess::Ok(num),
    Ok(_) => {
      print!(" The riddle is between 1 and {BOUNDARY}.\n\t");
      Guess::Err
    }
    Err(_) => {
      print!(" The guess could be only the number between 1 and {BOUNDARY}.\n\t");
      Guess::Err
    },
  }
}

fn small() {
  print!(" Too small!");
}

fn large() {
  print!(" Too big!");
}

fn equal(answer: u32, tries: u32) {
  print!("\n {answer} is the correct answer, congratulations!");
  print!("\n It took You {tries} tries to guess correctly, but can you do better?");
  footer();
}

fn main() {
  header();
  let secret_number = generate_number();
  let mut tries = 0;
  loop {
    next_round(&mut tries);
    match get_guess() {
      Guess::Ok(guess) if guess < secret_number => small(),
      Guess::Ok(guess) if guess > secret_number => large(),
      Guess::Ok(_) => { equal(secret_number, tries); return },
      Guess::Err   => continue,
    };
  }
}
