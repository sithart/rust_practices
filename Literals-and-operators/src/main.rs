fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1u32 - 2);

    // Short circuting boolean logic
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operator
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}",  0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}",  0b0011u32 ^ 0b0101);

    // Use underscore to improve readability
    println!("One million is written as {}", 1_000_000u32);

}