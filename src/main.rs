use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    x 
}



fn main() {
    let mut soma: i32 = 0;
    let mut valor_entrada: String = String::new();
    io::stdin().read_line(&mut valor_entrada).expect("Failed to read line");
    let mut valor_i32: i32 = convert_to_int(&valor_entrada);
    while valor_i32 != 0 {
        let r = valor_i32 % 10;
        soma = soma + r;
        valor_i32 = valor_i32 / 10;
    }
    println!("Valor da soma dos digitos: {}", soma);
}

