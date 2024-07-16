fn guess() {
    use std::io;
    use rand::Rng;
    use std::cmp::Ordering;

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!\nYour guess: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn main() {
    guess()
}
