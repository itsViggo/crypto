# crypto
Something Awesome project by z5255879 (Vicknesh Ravikumar)

This is a cryptography tool built in Rust that, by the end, will be able to encrypt and decrypt using a variety of crypto algorithms. To build the tool, cd into the directory and run `cargo build` (Note: you must have rust installed). To use the tool, navigate into the target/debug directory and use one of the following commands.

## Installation
If you haven't already, install Rust. The following command can be used to install Rust on macOS, Linux and other Unix-like OS
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Once you've done this, make sure rustc has been added to your path. You can check by running the following and if it gives you a version number, you have succesfully added rustc to your PATH
```
$ rustc -V
rustc 1.50.0 (cb75ad5db 2021-02-10)
```
Once, you've done this, clone this repository onto your machine and then compile it using the following commands
```
$ git clone https://github.com/itsViggo/crypto.git
$ cd crypto/
$ rustc crypto.rs
```
Then, you can add the directory to the system's path so that you can run the crypto tool from anywhere on your device.
```
$ echo 'export PATH=$PATH:'$( pwd ) >> ~/.bash_profile
```
Once you've done this, logged out and logged back into bash, the program can be run from anywhere on your device
## Caesar encrypt
```
$ crypto caesar_encrypt [plaintext] [shift]
```
For example,
```
$ crypto caesar_encrypt 'Hello world!' 13
Uryyb jbeyq!
```
shift must be between -127 and 127 inclusive.
## Caesar decrypt
```
$ crypto caesar_decrypt [plaintext] [shift]
```
For example,
```
$ crypto caesar_decrypt 'Uryyb jbeyq!' 13
Hello world!
```
shift must be between -127 and 127 inclusive
## Simple substitution encrypt
```
$ crypto substitution_encrypt [plaintext] [original] [translation]
```
For example,
```
$ crypto substitution_encrypt "defend the east wall of the castle" "abcdefghijklmnopqrstuvwxyz" "phqgiumeaylnofdxjkrcvstzwb"
giuifg cei iprc tpnn du cei qprcni
```
original and translation must be same size with whatever character is in the nth position in the original being encrypted into whatever character is in the nth position in the translation
## Simple substitution decrypt
```
$ crypto substitution_decrypt [plaintext] [original] [translation]
```
For example,
```
$ crypto substitution_decrypt "giuifg cei iprc tpnn du cei qprcni" "abcdefghijklmnopqrstuvwxyz" "phqgiumeaylnofdxjkrcvstzwb"
defend the east wall of the castle
```
original and translation must be same size with whatever character is in the nth position in the original being decrypted from whatever character is in the nth position in the translation
## Vigenere encrypt
```
$ crypto vigenere_encrypt [plaintext] [key]
```
For example,
```
$ crypto vigenere_encrypt "hello world" LEMON
sixzb hsdzq
```
The key must be in all capital letters
## Vigenere decrypt
```
$ crypto vigenere_decrypt [ciphertext] [key]
```
For example,
```
$ crypto vigenere_encrypt "wazxb lkfxq" LEMON
hello world
```
The key must be in all capital letters
