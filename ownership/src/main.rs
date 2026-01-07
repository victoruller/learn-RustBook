fn main() {
// Exclusão n°1: saiu do escopo
    {
        let _s: String = String::from("String propriamente dita");
    }
    //    println!("{_s} é inválido aqui fora e o valor apontado por ele já foi excluído"); // <- erro de compilação

    loop {
        let _s: String = String::from("String propriamente dita");
        break;
    }
    //    println!("{_s} é inválido aqui fora e o valor apontado por ele já foi excluído"); // <- erro de compilação


// Exclusão N°2: Reatribuição de Owner
    let mut _real_string: String = String::from("String propriamente dita");
    _real_string = String::from("Valor diferente");
    // O valor "String propriamente dita" apontado anteriormente já foi excluido automaticamente da memória

    //Em outras linguagens ele ficaria esquecido.
    

    
    let mut _real_string: String = String::from("Fora");
    {
        _real_string = String::from("Dentro");
    }
    println!("O valor é: {}", _real_string); // Dentro

    {
        let _real_string = String::from("Dentro Dentro");
    }
    println!("O valor 2 é: {}", _real_string); // Dentro





// Exclusão Nº3: BORROW SITUATIONS

    // -- Situação 1 (OK) -- Borrow com variável de tamanho fixo.
    let _litera_int: u32 = 10;

    let _x: u32 = _litera_int;

    println!("{_litera_int} ainda pode ser usado (e {_x} também)"); // Variáveis de tamaho fixo tem `copy` implementado, que possibilita isso.






    // -- Situação 2 (OK) -- Borrow de ponteiro string e mutable
    let mut _literal_string: &str = "Ponteiro para string"; // Owner mantém o tamanho mesmo após a string ser modificada;

    let mut _x: &str = _literal_string;

    println!("{_literal_string} ainda pode ser utilizada (e {_x} também.)");
    _x = "h";




    // -- Situação 3 (ERRO) -- Borrow de owner
    let _real_string: String = String::from("String propriamente dita"); // String pode modificar seu tamanho, visto ser um tipo complexo;

    let _x: String = _real_string;

    //    println!("{realString} Foi movida para dentro de X ({x}), realString não existe mais"); // <- erro de compilação (borrow of moved value)




    // -- Situação 4 (ERRO) -- Borrow de owner como parâmetro para função
    let _real_string: String = String::from("String propriamente dita");

    function(_real_string);

    //    println!("{realString} Foi movida para 's' da função, não existe mais"); // <- erro de compilação (borrow of moved value)

    let _real_string: String = String::from("String propriamente dita");
    println!("{_real_string}");
    println!("{_real_string}");




    // -- Situação 5 (ERRO) -- Shadowing o owner

    let _real_string: String = String::from("String propriamente dita");
    let _real_string = 13; // A string do owner foi deletada
    // para provar, crie um objeto com println na função drop()

    let real_string: String = String::from("String propriamente dita");
    let pointer = &real_string;
    let _real_string = 12; // A string continua em memória, pois está viva por meio do pointer

    println!("Dentro do pointer há: {}", pointer);


    



    // -- Situação 7 (OK) -- Usando Referência
    let _real_string : String = String::from("String propriamente dita");
    let _x = &_real_string;
    println!("Referência não exclui o owner: {_real_string}");


    // -- Situalção 8 (OK) -- Usando clone()
    let _real_string: String = String::from("String propriamente dita"); // String pode modificar seu tamanho, visto ser um tipo complexo;

    let _x: String = _real_string.clone();

    println!("clone de {_real_string} foi movido para dentro de X ({_x}), realString ainda existe");
}

fn function(s: String) {
    println!("{s}");
}
