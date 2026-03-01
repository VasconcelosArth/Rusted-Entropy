use std::io;

mod password;

use password::generator;

fn options(){
    println!("Options:");
    println!("1 - Generate password");
    println!("2 - Password entropy check");
    println!("0 - Quit");
}

fn main() {
    'main_loop: loop {
        println!();

        let mut op = String::new();

        options();
        println!("Enter an option: ");

        io::stdin()
            .read_line(&mut op)
            .expect("Failed to read line!");

        let op: u8 = op.trim().parse().expect("Please type a number!");

        match op {
            1 => generator::pass_gen(),
            0 => break 'main_loop,
            _ => println!("Invalid option!")
        }
    }

    println!("Goodbye!");
}
