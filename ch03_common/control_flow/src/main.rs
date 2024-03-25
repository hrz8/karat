fn main() {
    // error, the if statement should be bool (no truthy thing)
    // let number = 3;
    // if number {
    //     println!("number was three");
    // }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // returning value using if
    // error, expected same data type in if expression
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");
    // working
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // `loop`
    // loop {
    //     println!("again!");
    // }

    // `loop` can have return value with `break`
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 100 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");

    // `loop labels`
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 7 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // `while`
    let mut number = 3;
    while number > 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF 🚀!!!");

    // read array (error prone; possible to panic if the index value or test condition is incorrect)
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // `for`
    // let a = [10, 20, 30, 40, 50];
    let a = 10..=50;
    for element in a {
        println!("using for; the value is: {element}");
    }

    // countdown, will start from 3, using rev() (reverse)
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF 🚀!!!");

    // finish on 4 (because of =)
    for number in 1..=4 {
        println!("{number}!");
    }
    println!("LIFTOFF 🚀!!!");
}
