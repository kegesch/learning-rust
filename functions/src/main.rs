fn main() {
    println!("Hello, world!");

    another_function(2);
    function_with_new_context();
    function_with_return();
}

fn another_function(x: i32) {
    println!("Another function. With value: {}", x);
}

fn function_with_new_context() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// no need for return
fn function_with_return() -> char {
    'J' // no semicolon means its not a statement
}