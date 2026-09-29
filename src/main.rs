use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("\t### Punching Table ###");

    loop {
        let mut rng = rand::thread_rng();
        let round = rng.gen_range(0..=3);

        let op: [&str; 4] = ["+", "-", "*", "/"];

        let (n1, n2) = if op[round] == "/" {
            let n2 = rng.gen_range(1..=12);
            let answer = rng.gen_range(1..=12);
            (n2 * answer, n2)
        } else {
            (
                rng.gen_range(1..=12),
                rng.gen_range(1..=12),
            )
        };

        let current_result = match op[round] {
            "+" => n1 + n2,
            "-" => n1 - n2,
            "*" => n1 * n2,
            "/" => n1 / n2,
            _ => unreachable!(),
        };

        println!("\x1b[1;31mWhat is {} {} {}\x1b[0m", n1, op[round], n2);

        print!("=  ");
        io::stdout().flush().unwrap();

        let mut user_answer = String::new();

        io::stdin()
            .read_line(&mut user_answer)
            .expect("Failed to read line");

        let user_answer: i32 = user_answer.trim().parse().unwrap();

        if user_answer == current_result {
            println!("Correct!");
        } else {
            println!("Incorrect! The answer was {}", current_result);
        }

        println!("Wanna continue? [y/n]");

        let mut con = String::new();

        io::stdin()
            .read_line(&mut con)
            .expect("Failed to read line");

        match con.trim().to_lowercase().as_str() {
            "n" => break,
            "y" | "" => continue,
            _ => println!("Please enter y or n."),
        }
    }

    println!("Bye!");
}