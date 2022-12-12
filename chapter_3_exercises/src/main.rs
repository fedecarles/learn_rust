fn main() {
    println!("Celsius-Farenheit converter");
    temp_converter(12.0, 'C');
    temp_converter(53.6, 'F');
    println!("---\n");
    println!("Fibonacci Sequence");
    println!("{}", fibonacci(5));
    println!("{}", fibonacci(10));
    println!("---\n");
    println!("Twelve days of Christmas");
    twelve_days();
}

// Celsius-Farenheit converter
fn temp_converter(degree: f32, temp_type: char){
    if temp_type == 'C' {
        let farenheit = (degree*1.8) + 32.0;
        println!("{degree}º{temp_type} = {farenheit}ºF");
    } else if temp_type == 'F' {
        let celsius = 5.0/9.0 * (degree - 32.0);
        println!("{degree}º{temp_type} = {celsius}ºC");
    } else {
        println!("Please input a valid temperature.");
    }
}

// fibonacci sequence
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Twelve days of Christmas
fn twelve_days() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["Twelve drummers drumming", "Eleven pipers piping", "Ten lords a-leaping", "Nine ladies dancing", "Eight maids a-milking", "Seven swans a-swimming", "Six geese a-laying", "Five golden rings", "Four calling birds", "Three french hens", "Two turtle doves", "A partridge in a pear tree", ""];

    let n = gifts.len() - 1;

    let mut index = 1;
    for d in days {
        println!("On the {d} day of Christmas my true love gave to me:");
        let gift = &gifts[n - index .. n];
        for g in gift{
            println!("{g}");
        }
        index += 1;
    };

}
