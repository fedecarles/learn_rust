// create new vector
let v: Vec<i32> = Vec::new();

// create vector with initial values
// use macro vec! to have Rust guess the data type
let v = vec![1, 2, 3];

// update vector
// push adds an item to the vector
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);

// reading elements in vector
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// iterating over a vector
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

