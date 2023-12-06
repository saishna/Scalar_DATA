fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    let t = true;
    let f: bool = false; 
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // let (x, y, z) = tup;

    // println!("The value of x is: {x}");
    let x: (i32, f64, u8) = (500, 6.4, 1);
    
    let five_hundred = x.0;
    
    let six_point_four = x.1;
    
    let one = x.2;
    println!("five_hundred:{five_hundred}") ;
    println!("six_4:{six_point_four}") ;
    println!("one:{one}");

    // let a=[1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let b: [i32; 5] = [1, 2, 3, 4, 5]; //; says array of 5 
    // let a = [3; 5]; //it means its array of 5 with three
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];


 }


