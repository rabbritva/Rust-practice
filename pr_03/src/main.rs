use std::collections::HashMap;
use std::io;
use std::fs;
fn main() {
    ex_1();
    ex_2();
    ex_3();
    ex_4();
    ex_5();
    ex_6();
    ex_7();
}

fn ex_1()->(){
    println!("\n***\nEX 1:\n");
    let v1:Vec<i16> = vec![1,2,3,4,5];
    let v2:Vec<i16> = vec![6,7,8,9,10];
    println!("v1: {:?}\nv2: {:?}", v1, v2);
    fn extend(a:&Vec<i16>, b:&Vec<i16>) -> Vec<i16>{
        let mut rv:Vec<i16> = Vec::new();
        rv.extend(a);
        rv.extend(b);
        rv
    }
    let rv:Vec<i16> =  extend(&v1, &v2);
    println!("rv: {:?}", rv);
}

fn ex_2() -> (){
    println!("\n***\nEX 2:\n");
    let mut v1:Vec<i16> = vec![1,2,3,4,5];
    let mut v2:Vec<i16> = vec![1,2,3,4,5,6];
    println!("v1: {:?}\nv2: {:?}", v1, v2);
    fn func(x:&mut Vec<i16>) ->&mut Vec<i16>{
        if x.len()%2!=0{
            x.remove(x.len()/2);
        }
        else{
            let a:i16 = x.pop().unwrap();
            x.push(a*a);
        }
        x
    }
    println!("func(v1) = {:?}",func(&mut v1));
    println!("func(v2) = {:?}", func(&mut v2));
}

fn ex_3() -> (){
    println!("\n***\nEX 3:\n");
    let mut list1:Vec<&str> = vec!["Абрамовских Дмитрий", "Аюпова Вероника","Бадгиева Алина", "Бужангорский Матвей"];
    let mut list2:Vec<&str> = vec!["Абрамовских Дмитрий", "Аюпова Вероника","Бадгиева Алина", "Бужангорский Матвей",
                                    "Паршукова Наталья Борисовна"];
    fn checker(list:&mut Vec<&str>) -> String{
        const VIP:&str = "Паршукова Наталья Борисовна";
        if list.contains(&VIP){
            "Список полон".to_string()
        }
        else{
            list.push(VIP);
            "Теперь список полон".to_string()
        }
    }
    println!("list1: {:?}\nlist2: {:?}", list1, list2);
    println!("checker(list1): {}", checker(&mut list1));
    println!("checker(list2): {}", checker(&mut list2));
}

fn ex_4()->(){
    println!("\n***\nEX 4:\n");
    let dist:Vec<i16> = vec![10,7,15,5,9];
    print!("dist: {:?}\n", dist);
    let mut s: i16 = 0;
    for x in 0..dist.len(){
        s += dist[x];
        print!("day {}: dist = {} km\n", x+1, s);
    }
}

fn ex_5()->(){
    println!("\n***\nEX 5:\n");
    let mut msg:String = String::from("Рейс № 337 вылетает в 12:15 в Барнаул.");
    println!("msg: {}", msg);
    msg = msg.replace("Рейс", "Внимание!!! Рейс").replace("12:15", "14:00").replace("Барнаул", "Абакан");
    println!("new_msg: {}", msg);
}

fn ex_6()->(){
    println!("\n***\nEX 6:\n");
    let coords:(f32,f32) = (400.0,270.0);
    let nach:f32 = 200.0;
    let teck:f32 = 50.0;
    fn func(c:(f32,f32), nach: f32, teck:f32) -> (f32,f32){
        (c.0+ (c.0/nach)*teck, c.1+(c.1/nach)*teck)
    }

    println!("old coords: {:?}", coords);
    println!("new coords: {:?}", func(coords, nach, teck));
}

fn ex_7()-> (){
    println!("\n***\nEX 7:\n");
    let mut planets:HashMap<String, f32> = HashMap::new();
    let content = fs::read_to_string("file.txt").expect("Error read file");
    let content: Vec<&str> = content.split("\n").collect();
    for x in 1..content.len(){
        let words: Vec<&str> = content[x].split_whitespace().collect();
        if words.len() > 1{
            planets.insert(words[0].to_lowercase(), words[1].parse().expect("Не удалось считать число"));
        }
        
    }

    println!("Введите планету отправки и ваш возраст (через пробел): ");
    let mut content = String::new();
    io::stdin().read_line(&mut content).expect("Не удалось считать строку");
    let content: Vec<&str> = content.trim().split_whitespace().collect();

    if content.len()<2{
        println!("Введите корректные значения!");
        return ;
    }

    print!("Ваш возраст в годах ");
    let planet = content[0];
    let mut age: f32 = content[1].parse().expect("Введите целое количество лет!");
    age = age/planets[planet.to_lowercase().as_str()];
    let age: i16 = age as i16;

    match planet{
        "меркурий" | "Меркурий" => print!("Меркурия ="),
        "венера" | "Венера"=> print!("Венеры ="),
        "земля" | "Земля"=> print!("Земли ="),
        _ => print!("{planet}а =")

    }
    if 10<age && age<21 {
        print!(" {age} лет\n");
    }
    else if age%10==1 || age%10==2 || age%10==3 || age%10==4{
        print!(" {age} года\n")
    }
    else{
        print!(" {age} лет\n")
    }
}

