// https://kata-log.rocks/fizz-buzz-kata

const MAX: usize = 100;

fn if_fizz_buzz() {
    for n in 1..=MAX {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn match_fizz_buzz() {
    for n in 1..=MAX {
        match(n%3, n%5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", n)
        }
    }
}

// run tests with std output: cargo test -- --nocapture
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_if_fizz_buzz() {
        if_fizz_buzz();
    }

    #[test]
    fn test_match_fizz_buzz() {
        match_fizz_buzz();
    }
}