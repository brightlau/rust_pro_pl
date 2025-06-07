mod guess;

use guess::core::guess_fn;

pub fn guessing() {
    println!("beg, guessing...");
    guess_fn();
    println!("end, guessing...")
}
