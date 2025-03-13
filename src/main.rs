fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments!");
    }
    let mode = &args[1];

    if mode.trim() == "cipher" {
        let mut message = String::new();
        println!("Message to cipher: ");
        std::io::stdin()
            .read_line(&mut message)
            .expect("Problem with your message.");

        let bytes = message.trim().to_string().into_bytes();
        let mut binary = String::new();
        for byte in bytes {
            binary.push_str(format!("{:08b}", byte).as_str());
        }

        let mut laugh = String::new();
        for digit in binary.chars() {
            if digit == '0' {
                laugh.push_str("a");
            } else if digit == '1' {
                laugh.push_str("x");
            } else {
                panic!("Something went wrong");
            }
        }

        println!("{laugh}");
    } else if mode == "decipher" {
        let mut laugh = String::new();
        println!("Message to decipher: ");
        std::io::stdin()
            .read_line(&mut laugh)
            .expect("Problem with your message.");

        let mut message = String::new();
        for letter in laugh.trim().chars() {
            if letter == 'а' || letter == 'a' {
                message.push_str("0");
            } else if letter == 'х' || letter == 'x' {
                message.push_str("1");
            } else {
                println!("Current letter: {letter}");
                panic!("Something went wrong");
            }
        }

        let mut trimmed_msg = message.trim().to_string();
        let letter_count = trimmed_msg.chars().count() / 8;
        let mut decrypted_message = String::new();
        for _ in 0..letter_count {
            let (letter, remaining) = trimmed_msg.split_at(8);
            let number = u32::from_str_radix(&letter, 2).unwrap();
            decrypted_message.push(char::from_u32(number).unwrap());
            trimmed_msg = remaining.to_string();
        }

        println!("{decrypted_message}");
    } else {
        panic!("wrong mode! use cipher or decipher");
    }
}
