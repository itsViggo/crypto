use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "caesar_encrypt" => println!("{}", caesar_encrypt(&args[2], match args[3].parse::<i8>() {
                Ok(i) => i,
                Err(_e) => panic!("Please ensure the shift is a number between -127 and 127 inclusive"),
            })),
        "caesar_decrypt" => println!("{}", caesar_encrypt(&args[2], -1 * match args[3].parse::<i8>() {
            Ok(i) => i,
            Err(_e) => panic!("Please ensure the shift is a number between -127 and 127 inclusive"),
        })),
        _ => println!("Invalid cipher"),
    }
}

fn caesar_encrypt(plaintext: &String, shift: i8) -> String{
    let shift = ((shift % 26) + 26) % 26;
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

fn shift_on_alphabet_starting_with(c: char, start: char, shift: i8) -> char {
    let c_ascii = c as u8;
    let start_ascii = start as u8;
    let old_alphabet_pos = (c_ascii - start_ascii) as u8;
    let new_alphabet_pos = ((((old_alphabet_pos as i16 + shift as i16) % 26) + 26) % 26) as u8;
    (new_alphabet_pos + (start as u8)) as char
}