use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Jogo Adivinhe o Número, entre 1 e 100");
    println!("Entre com um número para registrar o seu palpite: ");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(..) => continue,
        };

        println!("Você registrou o número: {}", guess);
        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Vai mais alto!"),
            cmp::Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
            cmp::Ordering::Greater => println!("Vai mais baixo!"),
        }
    }

    println!("O número secreto é: {}", secret_number);
}
