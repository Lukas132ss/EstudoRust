use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    x 
}


fn main() {
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Failed to read line");

    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Failed to read line");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("Number {} greater than Number {}", convert_to_int(&number1), convert_to_int(&number2));
    }else {
        println!("Number {} less than or equal to Number {}", convert_to_int(&number1), convert_to_int(&number2));
    }   
}

