mod io;
mod variable;
mod function;

use io::println::println_fn;
use variable::let_mut::let_mut_fn;
use variable::strings::strings_fn;
use function::associated_function::associated_fn;

pub fn tour_rust_fn() {
    println_fn();
    let_mut_fn();
    strings_fn();
    associated_fn();
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
