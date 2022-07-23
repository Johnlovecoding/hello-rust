use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("game started!");

    let number = rand::thread_rng().gen_range(1..10);

    loop {
        println!("now enter your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("try greater"),
            Ordering::Greater => println!("try less"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    }
    

    

}
