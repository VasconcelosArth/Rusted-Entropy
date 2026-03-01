use std::io;
use rand::Rng;


fn options(){
    println!("Password generator options:");
    println!("1 - Generate generic password");
    println!("2 - Generic custom password");
    println!("0 - Back");
}

fn generic_password_gen(){
    let mut password_len = String::new();
    let mut count = 0;
    let mut password: Vec<char> = Vec::new();

    println!("Enter the password lenght:");

    io::stdin()
        .read_line(&mut password_len)
        .expect("Failed to read password len!");

    let password_len: u8 = password_len.trim().parse().expect("Please type a number!");

    while count < password_len {
        password.push(rand::thread_rng().gen_range(48..=57u8) as char);

        count += 1;
    }

    let pass: String = password.into_iter().collect();

    println!();
    print!("Password generated: ");
    println!("{pass}");
}

fn custom_password_gen(){
    println!("Testing");
}

pub fn pass_gen(){
    
    'main_loop: loop {
        println!();

        let mut op = String::new();

        options();
        println!("Enter an option:");

        io::stdin()
            .read_line(&mut op)
            .expect("Failed to read line!");

        let op: u8 = op.trim().parse().expect("Please type a number!");

        match op {
            1 => generic_password_gen(),
            2 => custom_password_gen(),
            0 => break 'main_loop,
            _ => println!("Invalid option!")
        }
    }
}