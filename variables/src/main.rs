fn main() {
    let mut x = 5;
    const MAX_POINTS: u32 = 100_000;

    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is {}", y);

    // this is lit -> changing its type by shadowing it.
    let spaces = "   ";
    let spaces = spaces.len();

    // floats
    let x = 2.0;
    let y: f32 = 3.0;

    // how?
    //let x: i32 = 1 / 2.0;
    //let x2: f32 = 1 / 2.0;

    //println!("The value of x div is {}", x);
    //println!("The value of x2 div is {}", x2);

    let c = 'z';
    let z = 'ℤ';
    // yay unicode
    let heart_eyed_cat = '😻';

    // yay inbuilt tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // deconstruct
    let (x, y, z) = tup;

    // or

    let y = tup.1;
    println!("The value of y in tuple is {}", y);


    // arrays
    let a = [1, 2, 3, 4, 5];
    let a2 =  [3; 5]; // 5 times 3

    // accessing
    let el = a[0]; // zero-index-based . cool

    //let el = a[5]; // will compile but not run!


}
