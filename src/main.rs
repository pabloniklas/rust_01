use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el numero");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Tu numero secreto es: {secret_number}");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Vos ingresaste ==> {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muy pequeño"),
            Ordering::Greater => println!("Muy grande"),
            Ordering::Equal => {
                println!("Muy bien");
                break;
            }
        }
    }
}
