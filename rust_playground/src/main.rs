fn caesar_cipher(s: &str, shift: i32) -> String {
    let shift = ((shift % 26) + 26) % 26;

    s.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { 'A' } else { 'a' } as u32;
                let code = c as u32;
                let shifted = (code - base + shift as u32) % 26 + base;
                std::char::from_u32(shifted).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}

fn encrypt(text: &str, shift: i32) -> String {
    caesar_cipher(text, shift)
}

fn decrypt(text: &str, shift: i32) -> String {
    caesar_cipher(text, -shift)
}

fn encrypt_N_decrypt() {
    let encrypted = encrypt("HelloWorld", 3);
    let decrypted = decrypt(&encrypted, 3);
    println!("Encrypted: {}", encrypted); // KhoorZruog
    println!("Decrypted: {}", decrypted); // HelloWorld
}








//vars and mutability,
fn mains() {
    let x: i32 = 5;
    //x = 6; // error: cannot assign twice to immutable variable `x`
    println!("The value of x is: {x}");
    let mut y = 5; // NOTICE the type isn't written here but the compiler knows its an i32 and shows it
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    const MAX_POINTS: u32 = 100_000;
    // MAX_POINTS = 100_001; // error: cannot assign twice to immutable variable `MAX_POINTS`
    println!("The value of MAX_POINTS is: {MAX_POINTS}");
}

//data types
// Length	Signed	Unsigned
// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	isize	usize




//if
fn main() {
    let num = 1;
    if num {
        println!("inside if")
    }
}


//