const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // error
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // shadowing
    // let x = 5;
    // println!("The value of x is: {x}");
    // let x = 6;
    // println!("The value of x is: {x}");

    // `mut`
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // shadowing 2
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // change the type
    // `let` will redeclare the variable so its okay if change the type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // warning, variable does not need to be mutable
    // let mut spaces = "   ";
    // let spaces = spaces.len();
    // println!("The value of spaces is: {spaces}");

    // error
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("The value of spaces is: {spaces}");
}
