use std::env;

const usage: &str = "Usage: ./crypto caesar_encrypt|caesar_decrypt [plaintext|ciphertext] [shift]\nUsage: ./crypto substitution_encrypt|substitution_decrypt [plaintext|ciphertext] [original] [translation]\nUsage: ./crypto vigenere_encrypt|vigenere_decrypt [plaintext|ciphertext] [key]";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "caesar_encrypt" => {
            println!("{}", caesar_encrypt(args));
        },
        "caesar_decrypt" => {
            println!("{}", caesar_encrypt(args));
        },
        "substitution_encrypt" => {
            println!("{}", substitution_encrypt(args));
        },
        "substitution_decrypt" => {
            println!("{}", substitution_encrypt(args));
        },
        "vigenere_encrypt" => {
            println!("{}", vigenere_encrypt(args));
        },
        "vigenere_decrypt" => {
            println!("{}", vigenere_encrypt(args));
        },
        _ => println!("{}", usage),
    }
}

fn caesar_encrypt(args: Vec<String>) -> String{
    if args.len() != 4 {
        return usage.to_string();
    }
    let plaintext = &args[2];
    return match args[3].parse::<i8>() {
        Ok(i) => {
            let mut shift = ((i % 26) + 26) % 26;
            if args[1] == "caesar_decrypt" {
                shift = shift * -1
            }
            let mut output = String::new();
            for c in plaintext.chars() {
                if c.is_uppercase() {
                    output.push(shift_on_alphabet_starting_with(c, 'A', shift));
                } else if c.is_lowercase() {
                    output.push(shift_on_alphabet_starting_with(c, 'a', shift));
                } else {
                    output.push(c)
                }
            }
            output
        }
        Err(_e) => "Please ensure the shift is an integer between -127 and 127 inclusive".to_string()
    };
}

fn substitution_encrypt(args: Vec<String>) -> String{
    if args.len() != 5 {
        return usage.to_string()
    }
    let (original, conversion) = if args[1] == "substitution_encrypt" {
        (&args[3], &args[4])
    } else {
        (&args[4], &args[3])
    };
    if original.len() != conversion.len() {
        return "Please ensure original and conversion have the same number of characters".to_string()
    }
    let plaintext = &args[2];
    let mut output = String::new();
    for c in plaintext.chars() {
        if original.contains(c) {
            output.push(conversion.chars().nth(original.find(c).unwrap()).unwrap());
        } else {
            output.push(c)
        }
    }
    output
}

fn vigenere_encrypt(args: Vec<String>) -> String{
    if args.len() != 4 {
        return usage.to_string()
    }
    let plaintext = &args[2];
    let key = &args[3];
    for c in key.chars() {
        if !c.is_uppercase() {
            return "Please ensure key is in all capital letters".to_string()
        }
    }
    let mut output = String::new();
    let mut i = 0;
    for c in plaintext.chars() {
        let mut shift = key.as_bytes()[i % key.len()] as i8 - 'A' as i8;
        if args[1] == "vigenere_decrypt" {
            shift = shift * -1
        }
        if c.is_uppercase() {
            output.push(shift_on_alphabet_starting_with(c, 'A', shift));
            i += 1;
        } else if c.is_lowercase() {
            output.push(shift_on_alphabet_starting_with(c, 'a', shift));
            i += 1;
        } else {
            output.push(c)
        }
    }
    output
}

fn shift_on_alphabet_starting_with(c: char, start: char, shift: i8) -> char {
    let c_ascii = c as u8;
    let start_ascii = start as u8;
    let old_alphabet_pos = (c_ascii - start_ascii) as u8;
    let new_alphabet_pos = ((((old_alphabet_pos as i16 + shift as i16) % 26) + 26) % 26) as u8;
    (new_alphabet_pos + (start as u8)) as char
}