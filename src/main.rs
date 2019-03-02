use ferris_says::say;
use std::io::{stdin, stdout, BufWriter};

fn main() {
    let stdin = stdin();
    let mut message = String::new();
    stdin.read_line(&mut message).expect("Failed to readline");
    let stdout = stdout();
    let width = 24;
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
