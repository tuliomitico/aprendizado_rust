use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
fn main() -> () {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Erro ao ler a");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Erro ao ler b");
    let mut divisao = 0;
    let mut i = 1; 
    while i < convert_to_int(&a) && i < convert_to_int( &b) {
        if convert_to_int( &a) % i == 0 && convert_to_int( &b) % i == 0 {
            divisao = i;
        }
        i += 1;
    }
    println!("O maior divisor comum eh {}",divisao);
}