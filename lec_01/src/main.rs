use std::io::{self, Write};

fn main() {

    // first(); //01
    // second() //02
    // third(); //03
    fourth(); //04
}

const MSG_DIGIT: &str = "Введите целое число";
const MSG_IN: &str = "Failed to read line";
const MSG_FLUSH: &str = "Flush ERROR";

fn first() -> (){
    let mut name: String = String::new();
    print!("Введите ваше имя: ");
    io::stdout().flush().expect("!");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Hello, {}!", name.trim());
}

fn second() -> (){
    let mut input: String = String::new();
    print!("Введите два числа: \na = ");
    io::stdout().flush().expect("!");
    let msg: String = "Введите целое число".to_string();
    io::stdin().read_line(&mut input).expect("Failed read line");
    let a: i32 = input.trim().parse().expect(&msg);
    input.clear();
    print!("b = ");
    io::stdout().flush().expect("!");
    io::stdin().read_line(&mut input).expect("Failed read line");
    let b: i32 = input.trim().parse().expect(&msg);
    input.clear();
    println!("Сумма введенных чисел: {}", a+b);
}

fn third() -> (){
    let mut input: String = String::new();
    println!("Введите два числа:");
    print!("a = ");
    io::stdout().flush().expect("Flash error");
    io::stdin().read_line(&mut input).expect(&MSG_IN);
    let a: i32 = input.trim().parse().expect(&MSG_DIGIT);
    input.clear();
    print!("a = ");
    io::stdout().flush().expect("Flash error");
    io::stdin().read_line(&mut input).expect(&MSG_IN);
    let b: i32 = input.trim().parse().expect(&MSG_DIGIT);
    input.clear();
    println!("a - b = {}", a-b);
}

fn fourth() -> (){
    let mut input: String = String::new();
    println!("Введите два числа: ");
    print!("a = ");
    io::stdout().flush().expect(&MSG_FLUSH);
    io::stdin().read_line(&mut input).expect(MSG_IN);
    let a: i32 = input.trim().parse().expect(MSG_DIGIT);
    input.clear();

    print!("b = ");
    io::stdout().flush().expect(MSG_FLUSH);
    io::stdin().read_line(&mut input).expect(MSG_IN);
    let b: i32 = input.trim().parse().expect(MSG_DIGIT);
    input.clear();

    println!("a * b = {}", a*b);
}