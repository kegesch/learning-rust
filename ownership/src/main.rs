use String;

fn main() {
    // this is a string literal which can not be mutated!
    let s = "hello";

    // this is a String which can also not be mutated!
    let s1 = String::from("hello");

    //takes_ownership(s1); // s1 moves into the function and is invalid after that.
    // that goes for every variable which does not have the Copy trait

    does_not_take_ownership(&s1);

    // but this can
    let mut s2 = String::from("hello");

    //s2 = s1; this will move the value!!!!
    println!("{}", s1); // should not work!
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn does_not_take_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
