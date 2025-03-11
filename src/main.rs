// Input words in plain english
// Transfer them to binary
// Transfer binary to laughing
// Output laughing

fn main() {
    let mut mode = String::new();
    println!("Choose your mode");
    std::io::stdin()
        .read_line(&mut mode)
        .expect("Problem with your mode.");

    if mode.trim() == "text" {
        let mut message = String::new();
        println!("Message to encrypt: ");
        std::io::stdin()
            .read_line(&mut message)
            .expect("Problem with your message.");

        // message.pop();
        let bytes = message.trim().to_string().into_bytes();
        let mut binary = String::new();
        for byte in bytes {
            binary.push_str(format!("{:08b}", byte).as_str());
            println!("Letter: {:08b}", byte);
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
    } else {
        let mut message = String::new();
        println!("Message to decrypt: ");
        std::io::stdin()
            .read_line(&mut message)
            .expect("Problem with your message.");

        let number = u32::from_str_radix(&message.trim(), 2).unwrap();
        let char = char::from_u32(number).unwrap();
        println!("{char}");
    }
}
