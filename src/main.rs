use ferris_says::say;
use std::io::{stdin, stdout, BufWriter};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut ferris = Ferris::new(24);
    ferris.ask();
}

struct Ferris {
    number: i32,
    width: usize
}

impl Ferris {
    fn new(width:usize) -> Ferris {
        Ferris {
            width: width,
            number: rand::thread_rng().gen_range(1, 11)
        }
    }

    fn say(&mut self, message: &str) {
        let mut writer = BufWriter::new(stdout());
        say(message.as_bytes(), self.width, &mut writer).unwrap();
    }
    
    pub fn ask(&mut self){
        self.say("Guess a number in 1-10");
        let stdin = stdin();
        loop {
            let mut input = String::new();
            stdin.read_line(&mut input).expect("Failed to readline");
            let guess: i32 = input.trim().parse().expect("Please type a number!");
            match guess.cmp(&self.number) {
                Ordering::Less => self.say("Too small!"),
                Ordering::Greater => self.say("Too big!"),
                Ordering::Equal => {
                    self.say("You win!");
                    break;
                }
            }
        }
    }
}