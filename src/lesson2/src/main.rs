use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let source_number = rand::thread_rng().gen_range(1, 100);

    loop {
        let mut guess = String::new();
        println!("plase input your number:");
        io::stdin().read_line(&mut guess).expect("err");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("your number is: {}", guess);
    
        match guess.cmp(&source_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }

}
