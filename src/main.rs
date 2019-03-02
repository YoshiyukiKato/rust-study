use ferris_says::say;
use std::io::{stdin, stdout, BufWriter};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut ferris = Ferris::new(1, 101);
    ferris.ask();
}

struct Ferris {
    from: i32,
    to: i32,
    number: i32,
    width: usize
}

impl Ferris {
    fn new(from: i32, to: i32) -> Ferris {
        Ferris {
            width: 24,
            from: from,
            to: to,
            number: rand::thread_rng().gen_range(from, to)
        }
    }

    fn say(&mut self, message: &str) {
        let mut writer = BufWriter::new(stdout());
        say(message.as_bytes(), self.width, &mut writer).unwrap();
    }
    
    pub fn ask(&mut self){
        self.say(&format!("Guess a number in {}-{}", self.from, self.to - 1)[..]);
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