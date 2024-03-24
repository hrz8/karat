use std::io;

fn main() {
    // panic on debug mode
    // overflow on cargo build --release
    // let num: u8 = 256;
    // println!("value: {}", num);

    let cc: &str = "halo";
    println!("value: {}", cc);

    let decimal: u32 = 98_222;
    println!("value: {}", decimal);

    let hex: u32 = 0xfe;
    println!("value: {}", hex);

    let bin: u32 = 0b1111_0000;
    println!("value: {}", bin);

    let byt1: u8 = b'a';
    let byt2: u8 = b'A';
    println!("value: {}, {}", byt1, byt2);

    let quotient = 56.7 / 32.2;
    println!("value: {}", quotient);

    // let truncated = (-5) as f64 / (3) as f64;
    // let truncated = -5.0 / 3.0;
    let truncated = -5 / 3;
    println!("value: {}", truncated);

    // let poo = '💩';
    let poo: char = '💩';
    println!("value: {}", poo);

    let tup1 = (500, 6.4, 1);
    println!("value: {}", tup1.0);
    println!("value: {}", tup1.1);

    let tup2 = (500, 6.4, 1);
    let (a, _b, c) = (500, 6.4, 1);
    println!("value: {}", tup2.0);
    println!("value: {}", tup2.1);
    println!("value: {}", a);
    println!("value: {}", c);

    let tup3: (i32, f64, u8) = (500, 6.4, 1);
    println!("value: {}", tup3.0);
    println!("value: {}", tup3.2);

    // with explicit type
    let months1: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("value: {}", months1[2]);

    let months2 = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("value: {}", months2[4]);

    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("value: {}", arr1[4]);

    // let arr2: [i32; 5] = todo!(); will panic
    let arr2 = [3; 5]; // 5 elements will be all 3
    println!("value: {}", arr2[4]);

    let available_arrays = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = available_arrays[index];

    println!("The value of the element at index {index} is: {element}");
}
