use std::env;
use num_primes::{Generator,Verification};
use num_bigint::{BigUint};
use rand::Rng;
use ring_algorithm::{is_coprime, extended_euclidian_algorithm};
use mod_exp::{mod_exp};

const usage: &str = "Usage: crypto caesar_encrypt|caesar_decrypt [plaintext|ciphertext] [shift]\nUsage: crypto substitution_encrypt|substitution_decrypt [plaintext|ciphertext] [original] [translation]\nUsage: crypto vigenere_encrypt|vigenere_decrypt [plaintext|ciphertext] [key]\nUsage: crypto rsa_keygen\nUsage: crypto rsa_encrypt|rsa_decrypt [plaintext|ciphertext] [public key|private key] modulus";

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
        "rsa_keygen" => {
            println!("{}", rsa_keygen(args));
        },
        "rsa_encrypt" => {
            println!("{}", rsa_encrypt(args));
        },
        "rsa_decrypt" => {
            println!("{}", rsa_decrypt(args));
        }
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

fn rsa_keygen(args: Vec<String>) -> String{
    if args.len() != 2 {
        println!("{}", usage.to_string())
    }
    let p = to_i128(Generator::new_prime(24).to_u32_digits());
    let q = to_i128(Generator::new_prime(24).to_u32_digits());
    let n = p * q;
    let m = (p - 1) * (q - 1);
    let e = find_coprime(m);
    let (_, d, _) = extended_euclidian_algorithm(e, m);
    format!("Public key: {0}, Private key: {1}, Modulus: {2}", e, d.abs(), n)
}

fn rsa_encrypt(args: Vec<String>) -> String {
    let mut plaintext = args[2].clone();
    let mut out = Vec::new();
    let mut outstring = String::new();
    let key = args[3].parse::<i128>().unwrap();
    let modulus = args[4].parse::<i128>().unwrap();
    for i in 0..plaintext.len()/3 {
        let n = to_i128(BigUint::from_bytes_le(plaintext[(i*3)..((i+1)*3)].as_bytes()).to_u32_digits());
        out.push(mod_exp(n, key, modulus));
    }
    if plaintext.len() % 3 != 0 {
        let n = to_i128(BigUint::from_bytes_le(plaintext[((plaintext.len()/3)*3)..].as_bytes()).to_u32_digits());
        out.push(mod_exp(n, key, modulus));
    }
    for num in out {
        let mut num_string = num.to_string();
        while (num_string.len() % 14 != 0) {
            num_string.insert(0, '0');
        }
        outstring.push_str(&num_string);
    }
    outstring
}

fn rsa_decrypt(args: Vec<String>) -> String {
    let key = args[3].parse::<i128>().unwrap();
    let modulus = args[4].parse::<i128>().unwrap();
    let ciphertext = args[2].clone();
    let mut out = String::new();
    for i in 0..ciphertext.len()/14 {
        let n = ciphertext[(i*14)..((i+1)*14)].parse::<i128>().unwrap();
        out.push_str(&from_i128_to_string(mod_exp(n, key, modulus)))
    }
    out
}

fn to_i128(u32_digits: Vec<u32>) -> i128 {
    let mut out = 0 as i128;
    let mut i = 1 as i128;
    for digit in u32_digits {
        out += i * (digit as i128);
        i += 1;
    }
    out
}

fn from_i128_to_string(num: i128) -> String {
    let mut remainder = num;
    let mut length = 1;
    let mut out = String::new();
    while 256_i128.pow(length) < num {
        length += 1;
    }
    while length != 0 {
        let quotient = remainder.div_euclid(256_i128.pow(length - 1)) as u8;
        remainder = remainder % 256_i128.pow(length - 1);
        length -= 1;
        out.insert(0, quotient as char);
    }
    out
}

fn find_coprime(m: i128) -> i128 {
    let mut rng = rand::thread_rng();
    let mut e:u64 = rng.gen();
    while !is_coprime(m, (e as i128)) {
        e = rng.gen();
    }
    e as i128
}

fn shift_on_alphabet_starting_with(c: char, start: char, shift: i8) -> char {
    let c_ascii = c as u8;
    let start_ascii = start as u8;
    let old_alphabet_pos = (c_ascii - start_ascii) as u8;
    let new_alphabet_pos = ((((old_alphabet_pos as i16 + shift as i16) % 26) + 26) % 26) as u8;
    (new_alphabet_pos + (start as u8)) as char
}