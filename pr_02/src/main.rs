use std::io::{self, Write};

const MSG_FLUSH: &str = "Flush ERROR";
const MSG_IN: &str = "Failed to read line";
const MSG_DIGIT: &str = "Введите целое число";
fn main() {

    first(); //01
    second(); //02
    third(); //03

    let mut p:String = String::new();
    io::stdout().flush().expect(&MSG_FLUSH);
    io::stdin().read_line(&mut p).expect(&MSG_IN);
}

fn first() -> (){
    let a: i32 = 25;
    let b: i32 = 20;
    let op = "+";
    match op {
        "+" => println!("{}", a+b),
        "-" => println!("{}", a-b),
        _ => todo!()
    }

}

fn draw_matrix(n:i16, m:i16, sym:String) -> (){
    for _i in 0..n{
        for _j in 0..m{
            print!("{} ", sym.trim());
        }
        println!();
    }
}

fn second()->(){
    let mut sym: String = String::new();
    print!("Введите символ: ");
    io::stdout().flush().expect(&MSG_FLUSH);
    io::stdin().read_line(&mut sym).expect(&MSG_DIGIT);
    let mut input: String = String::new();
    print!("Введите размерность (два числа через пробел): ");
    io::stdout().flush().expect(&MSG_FLUSH);
    io::stdin().read_line(&mut input).expect(&MSG_DIGIT);
    let mut parts = input.trim().split_whitespace();
    let m = parts.next().expect(&MSG_DIGIT).parse().expect(&MSG_DIGIT);
    let n = parts.next().expect(&MSG_DIGIT).parse().expect(&MSG_DIGIT);
    draw_matrix(n,m,sym.trim().to_string());
}

fn third()-> (){
    //01
    println!("\nЗадача 1:");
    let chr:char = 'A';
    if chr.is_lowercase(){
        println!("Регистр символа {}: нижний", chr);
    }
    else if chr.is_uppercase(){
        println!("Регистр символа {}: верхний", chr);
    }
    else{
        println!("{} не буква", chr);
    }

    //02
    println!("\nЗадача 2:");
    let num: i16 = 12000;
    let zeros = &num.to_string()[2..];
    println!("{}",zeros);

    //03
    println!("\nЗадача 3:");
    let arr: [i16;10] = [4,3,12,5,45,9,75,5,-55,66];
    println!("Массив: {:?}", arr);
    for i in 0..arr.len(){
        if arr[i]%5==0{
            println!("{}", arr[i]);
        }
    }

    //04
    println!("\nЗадача 4:");
    let arr:[i16; 6] = [1, 2, 0, 3, 4, 5];
    let  mut idx: isize = -1;
    for i in 0..arr.len(){
        if arr[i] == 0 {idx = i as isize; break;}
    }
    if idx!=-1{
        let idx = idx as usize;
        println!("{:?}", &arr[..idx]);
        println!("{:?}", &arr[idx+1..]);
    }
    else{
        println!("{:?}", arr);
    }

    //05
    println!("\nЗадача 5:");
    let string = "abc_abc_abc".to_string();
    let mut res: String = String::new();
    for i in string.chars(){
        if i == '_'{
            res.push(' ');
        }
        else {res.push(i);}
    }
    println!("{}", res);
    println!("{}", string.replace('_', " "));

    //06
    println!("\nЗадача 6: организовать считывание чисел в массив");
    let mut arr: [i16;10] = [0;10];
    let mut string:String = String::new();
    println!("Введите массив из 10 целых чисел: ");
    io::stdout().flush().expect(&MSG_FLUSH);
    io::stdin().read_line(&mut string).expect(&MSG_IN);
    let string = string.trim().to_string();
    let mut k = 0;
    for i in string.split_whitespace(){
        if i.to_string() != " " && k<10 {
            arr[k] = i.to_string().parse().expect(&MSG_DIGIT);
            k+=1;
        }
        else if k>=10 {break;}
    }
    println!("Ваш массив: {:?}", &arr);
}
