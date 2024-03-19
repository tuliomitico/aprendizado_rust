use std::io; /* importação de biblioteca de input/output padrão */

/* fn convert_to_int(data_input: & String) -> i32 { // função para converter valores string em inteiros
     let x = data_input.trim().parse::<i32>().unwrap();
     x
} */
fn main() -> () {
    let mut a = 15;
    let mut b = 40;
    while b != 0  // enquanto b for diferente de zero continuar no loop while
    {
    let temp = b; // armazene  o menor valor das duas variáveis na variável temp
    b = a % b; // armazene o resto da divisão do valor maior pelo menor
    a = temp; // armazene o valor menor em temp na variável a
    }
    println!("O maior divisor comum entre 15 e 40 eh: {}", a);
}