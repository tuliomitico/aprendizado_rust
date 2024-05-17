fn dobro(num: i32) -> i32 {
    return 2 * num;
}

fn maior(a: i32, b: i32) -> i32 {
    if a >= b{
        return a;
    }
    else{
        return b;
    }
}
fn alguma_fn(par_a: f32, par_b: i128) -> f32 {
    println!("Essa função devolve um valor flutuante");
    10.1
}

fn main() -> () {
    println!("O maoir numero entre 5 e 4 eh {}",maior(5,4));
    println!("O dobro de {} eh: {}",5,dobro(5));
}