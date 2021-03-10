# crypto
Something Awesome project by z5255879 (Vicknesh Ravikumar)

This is a cryptography tool built in Rust that, by the end, will be able to encrypt and decrypt using a variety of crypto algorithms. To build the tool, cd into the directory and run `cargo build` (Note: you must have rust installed). To use the tool, navigate into the target/debug directory and use one of the following commands.

## Caesar encrypt
```
$ ./crypto caesar_encrypt [plaintext] [shift]
```
For example,
```
$ ./crypto caesar_encrypt 'Hello world!' 13
Uryyb jbeyq!
```
shift must be between -127 and 127 inclusive.

## Caesar decrypt
```
$ ./crypto caesar_decrypt [plaintext] [shift]
```
For example,
```
$ ./crypto caesar_decrypt 'Uryyb jbeyq!' 13
Hello world!
```
shift must be between -127 and 127 inclusive
