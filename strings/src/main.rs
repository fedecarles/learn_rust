// The String type, which is provided by Rust’s standard library rather than
// coded into the core language, is a growable, mutable, owned, UTF-8 encoded 
// string type

// create new String
let mut s = String::new();

// convert initial data to String
let data = "initial contents";
let s = data.to_string();
// the method also works on a literal directly:
let s = "initial contents".to_string();

// or...
let s = String::from("initial contents");

// updating a string
let mut s = String::from("foo");
s.push_str("bar");

// string concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

// indexing strings
// this will fail. Rust does not allow for string indexing
let s1 = String::from("hello");
let h = s1[0];

// Rather than indexing using [] with a single number, you can use [] with a
// range to create a string slice containing particular bytes:
let hello = "Здравствуйте";
let s = &hello[0..4];

// iterating over strings
for c in "Зд".chars() {
    println!("{}", c);
}
// or...
for b in "Зд".bytes() {
    println!("{}", b);
}

