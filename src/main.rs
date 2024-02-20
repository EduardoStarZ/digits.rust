use std::io;

fn main() {
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).expect("could not read line");
    
    input = input.trim().replace(' ', "");

    if !input.chars().all(|x| x.is_numeric()) {
        println!("input is not numeric");
        return;
    }
    
    let digits : Vec<i32> = input.split_whitespace().map(|x| x.parse().expect("parsing error")).collect();
    let words : Vec<String> = convert_to_words(digits);

    for x in words.iter() {
        println!("{:?}", x);
    }
}

fn convert_to_words(chars : Vec<i32>) -> Vec<String> {
    let mut words : Vec<String> = Vec::new();
 
    for x in chars.iter() {
       match x {
           1 => words.push("One".to_owned()),
           2 => words.push("Two".to_owned()),
           3 => words.push("Three".to_owned()),
           4 => words.push("Four".to_owned()),
           5 => words.push("Five".to_owned()),
           6 => words.push("Six".to_owned()),
           7 => words.push("Seven".to_owned()),
           8 => words.push("Eight".to_owned()),
           9 => words.push("Nine".to_owned()),
           0 => words.push("Zero".to_owned()),
           _ => ()
       }
    }
     words   
}
