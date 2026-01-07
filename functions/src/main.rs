fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: i128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por gentileza, digite um número válido.");
            return;
        }
    };

    println!("O fatorial de {} é {}", input, acha_fatorial(input));

    let mut x = 5;
    altera_valor(&mut x);
}




fn acha_fatorial(fatorial: i128) -> i128{
    println!("Another function.");
    if fatorial == 1 {
        return 1;
    }
    return fatorial * acha_fatorial(fatorial - 1);
}

fn altera_valor(x: &mut i32) -> () {
    *x = 12;
}
