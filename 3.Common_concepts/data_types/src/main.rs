fn main() {
    // Integers
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)

    // Integer Overflow
    let f: u8 = 255; // If we set more than u8 can hold Rust will panic.

    // Floating point numbers
    let f = 2.0;
    let g: f32 = 3.0;

    // Addition
    let sum = 5 + 10;
    // Subtraction
    let difference = 95.5 - 4.3;
    // Multiplication
    let product = 4 * 30;
    // Division
    let quotient = 56.7 / 32.2;
    // Remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;

    let f: bool = false;

    // Character
    let c = 'z';
    let z = 'Z';

    // Tuple
    let tup = ("Let's Gett Rusty!", 100_000);
    let (channel, sub_count) = tup; // Destructuring a tuple.
    let sub_count = tup.1; // Access tuple value by index.

    // Arrays
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1]; // Access value by index.
    let x = error_codes[3]; // Index out of bounds

    let byte = [0; 8]; // Create an array of values from 0 to 8.
}
