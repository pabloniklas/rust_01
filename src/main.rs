use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    println!("Adivina el numero");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Tu numero secreto es: {secret_number}");

    let mut guess = String::new();

    loop {
        io::stdin()
        .read_line(&mut guess)
        .expect("Error");
    
        let guess: u32 = guess.trim().parse().expect("Por favor ingresa tu numero:");

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
