


fn main() {

    let /*mut*/ hello_world = String::from("hello world");

    let len = hello_world.len();
    let _slice = &hello_world[0..5];  // 0 - 5
    //s.clear(); // <- (erro de compilação) - mutable borrow, sendo que há outros borrows futuros
    println!("{}", _slice);
    let _slice = &hello_world[..len]; // 0 - final
    let _slice = &hello_world[0..];   // 0 - final
    let _slice = &hello_world[..];    // 0 - final

    let hello = first_word(&hello_world);
    println!("{}", hello);


    // String Slice funciona também em Strings literais
    let hello_world_literal = "hello world";
    let hello_literal = first_word(&hello_world_literal);
    println!("{}", hello_literal);
}


fn first_word(s : &str) -> &str { // &str = (String Slice)

    let bytes = s.as_bytes(); // retorna um agrupamento com cada byte da string

    for (i, &item) in bytes.iter().enumerate() { // enumerate() retorna uma tupla com (index, referência para valor) de cada membro do iterador
        if item == b' ' { // Verifica se o byte é um bite que representa o caractere ' '
            return &s[0..i];
        }
    }

    return &s[..];
}