fn main() {
    // Integers
    println!("Integers");
    let x: u8 = 5;
    let y: i32 = 53432;

    println!("x is an unsigned 8 bit integer with value: {x}");
    println!("y is an signed 32 bit integer with value: {y}");
    println!("An integer is a number without a fractional component.
             Each variant can be either signed or unsigned and has an explicit
             size. Signed and unsigned refer to whether it’s possible for
             the number to be negative.");
    println!("---\n");

    // Floating points
    println!("Floating points");
    let x = 2.0; // default f64
    let y: f32 = 3.0; // f32
    println!("x is a default f64 floating point with value: {x}");
    println!("y is an declared f32 floating point with value: {y}");
    println!("also has two primitive types for floating-point numbers,
             which are numbers with decimal points. Rust’s floating-point
             types are f32 and f64, which are 32 bits and 64 bits in size,
             respectively. The default type is f64 because on modern CPUs
             it’s roughly the same speed as f32 but is capable of more
             precision. All floating-point types are signed.");
    
    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("---\n");

    // Boolean
    println!("Booleans");

    let t = true;
    let f:bool = false;

    println!("t is a boolean with value: {t}");
    println!("f is an explicit boolean with value: {f}");
    println!("As in most other programming languages, a Boolean type in Rust
    has two possible values: true and false. Booleans are one byte in size.");
    println!("---\n");

    // Character
    println!("Character");
    let c = 'z';
    let z:char = 'Z'; // with explicit annotation
    let heart_eyed_cat = '😻';

    println!("c is a char type with value:{c}");
    println!("z is an explicit char type with value:{z}");
    println!("heart_eyed_cat is an emoji char type with value:{heart_eyed_cat}");
    println!("Note that we specify char literals with single quotes,
             as opposed to string literals, which use double quotes.
             Rust’s char type is four bytes in size and represents a Unicode
             Scalar Value, which means it can represent a lot more than
             just ASCII.");
    println!("---\n");

    println!("Compound Types");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("tup  is a tuple of value:{tup:?}");
    let (x, y, z) = tup;
    println!("The value of y in tup is: {y}");

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("a is an array of numbers with value:{a:?}");
    println!("months is an array of strings with value:{months:?}");
    let first = months[0];
    let second = months[1];
    println!("months[0] is the first element of months: {first}");
    println!("months[1] is the second element of months: {second}");









}
