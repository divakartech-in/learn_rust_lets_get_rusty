use std::collection::Hashmap;

fn main() {
    let a = [1, 2, 3]; // Arrays

    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut v2 = vec![1, 2, 3, 4, 5];

    let third: &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];

    match &row[1] {
        SpreadSheetCell::Int(i: &i32) => println!("{}", i),
        _ => println!("Not an Integer!")
    };

    let mut s: String = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3: String = s1 + &s2;

    println!("{}", s1);

    let s4 = format!("{}{}", s1, s2);

    let hello: String = String::from("Hello");
    let c: char = hello[0]; // Not possible

    for b: u8 in "hello".bytes() {
        println!("{}", b);
    }

    for c: char in "hello".chars() {
        println!("{}", c);
    }

    // Hashmaps

    let blue: String = String::from("Blue");
    let yellow: String = String::from("Yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name: String = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);

    for (key: String, value: &i32) in &scores {
        println!("{}: {}", key, value);
    }
}
