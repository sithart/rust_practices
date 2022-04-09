fn main(){
    let company_string = "Quanta"; // string type
    let rating_float   = 4.5;      // float type
    let is_growing_boolean = true; // boolean type
    let icon_char      = '♥';      // Unicode character type

    println!("Company Name is: {}", company_string);
    println!("Company rating 5 on is: {}", rating_float);
    println!("Company is growing: {}", is_growing_boolean);
    println!("Company icon is: {}", icon_char);


    // signed and unsigned integers
    let result = 10;               // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;
    println!("result value is {}", result);
    println!("sum is {} and age is {}", sum,age);
    println!("mark is {} and count is {}", mark, count);

}
