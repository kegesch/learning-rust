fn main() {
    let number = 4;

    if number == 4 {
        println!("{}", 4);
    } else if number > 4 {
        println!("number is bigger than 4");
    } else {
        println!("Not four: {}", number);
    }

    let cond = true;
    let x = if cond {1} else {0}; // if is an expression with return value!!!
    // but both branches needs to have the same return type

    loopi(4);
    let a = (1..4).rev(); // fast array struct construction;
    iter([1, 2, 3, 4]);

    println!("Fibonacci from 5 is {}", fib(5));
}

fn loopi(number: i32) {
    let mut n = 0;
    let fin = loop { // woohoo also an expression;
        n += 1;
        if(n == number) {
            break n;
        }
    };
    println!("Loppi reached {} in {} steps", number, fin);
}

fn iter(array: [i32; 4]) {
    for a in array.iter() {
        println!("{}", a);
    }
}

fn fib(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
