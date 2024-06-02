fn main() {
    let x = 5;
    let x = x + 5;
    {
        let x = x * 2;
        println!("Scope of x {x}");
    }
    let number: u8 = 250;
    println!("number {number}");

    println!("Outside scope {x}");
    let y = addition(5, 10);
    println!("{y}");
}

fn addition(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtraction(x: i32, y: i32) -> i32 {
    return x - y;
}

fn multiplication(x: i32, y: i32) -> i32 {
    return x * y;
}

fn division(x: i32, y: i32) -> i32 {
    return x / y;
}
