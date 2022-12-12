fn main() {
    // Mutable
    println!("Mutable Variables");
    let mut x = 5; // mut makes a variable mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value if x is: {x}. It can change because is muttable");
    println!("---\n");

    // Constants
    println!("Constants");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS is a constant of value: {THREE_HOURS_IN_SECONDS}");
    println!("Rust naming convention for constants is to use all uppercase with
             underscores between words. Constants are valid for the entire
             time a program runs, within the scope they were declared in.");
    println!("---\n");
    println!("Shadowing Variables");

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y in the outer scope is: {y}");

    println!("We can shadow a variable by using the same variable’s name
             and repeating the use of the let keyword. Shadowing is different
             from marking a variable as mut, because we’ll get a compile-time
             error if we accidentally try to reassign to this variable without
             using the let keyword. By using let, we can perform a few
             transformations on a value but have the variable be immutable
             after those transformations have been completed.")
    println!("---\n");
}
