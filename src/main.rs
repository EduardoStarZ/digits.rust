use std::io;

fn main() {
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).expect("could not read line");
    
    input = input.trim().replace(' ', "");

    if !input.chars().all(|x| x.is_numeric()) {
        println!("input is not numeric");
        return;
    }
    
    let digits : Vec<char> = input.chars().collect();
    let words : Vec<String> = convert_to_words(digits);

    for x in words.iter() {
        println!("{:?}", x);
    }
}

fn convert_to_words(chars : Vec<char>) -> Vec<String> {
    let mut words : Vec<String> = Vec::new();
 
    for x in chars.iter() {
       match *x as i32 {
           1 => words.push("One".to_string()),
           2 => words.push("Two".to_string()),
           3 => words.push("Three".to_string()),
           4 => words.push("Four".to_string()),
           5 => words.push("Five".to_string()),
           6 => words.push("Six".to_string()),
           7 => words.push("Seven".to_string()),
           8 => words.push("Eight".to_string()),
           9 => words.push("Nine".to_string()),
           0 => words.push("Zero".to_string()),
           _ => ()
       }
    }
     words   
}
