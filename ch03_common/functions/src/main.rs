fn five() -> i32 {
    5
}

fn main() {
    // error
    // let x = (let y = 6);
    // println!('{x}');

    // error
    // let x = let y = 6;
    // println!('{x}');

    // let x = y = 6;
    // println!('{x}');

    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");

    let fv = five();
    println!("{fv}");
    let sx = plus_one(fv);
    println!("{sx}");
}

fn plus_one(a: i32) -> i32 {
    a + 1
}