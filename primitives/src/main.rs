fn main() {
    let logical: bool = true;

    let a_flot: f64 = 1.0; // Regular annotation
    let an_integer =  5i32; // Suffix annotation

    let default_float = 3.0; // f64
    let default_integer = 7; //i32

    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variables can be changed
    let mut mutable = 12; //Mutable i32
    mutable = 21;

    // Error! The type of variable can't be changed
    mutable = true;

    // Variables can be overwritten with shadowing
    let mutable = true;

}
