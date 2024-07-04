fn main() {
    // vector
    let v: Vec<i32> = Vec::new();
    println!("vector: {v:?}");

    let v = vec![1, 2, 3, 4, 5];
    println!("vector: {v:?}");

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("mut vector: {v:?}");

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("does_not_exist: {does_not_exist:?}");

    // important!!!
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    let v = vec![100, 200, 300];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 200, 300];
    for i in &mut v {
        *i += 100;
        println!("{i}");
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("item: {i:?}");
    }

    // string
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("foo");
    let bar = "bar";
    s.push_str(bar);
    println!("bar is {bar}");

    let mut s = String::from("lo");
    s.push('l');

    let hello = String::from("hello, ");
    let world = String::from("world!");
    let helloworld = hello + &world;
    println!("{helloworld}");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tictactoe = format!("{tic}-{tac}-{toe}");
    println!("tictactoe: {}", tictactoe);

    // let s = String::from("hello");
    // let h = s[0];

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }


    // hash-map
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let color = String::from("Favorite color");
    let color_v = String::from("White");

    let mut map = HashMap::new();
    map.insert(color, color_v);
    // println!("{color}, {color_v}");


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 40);
    println!("{scores:?}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(80);
    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
    
}




// hash-map