fn main() {
    // Sair do escopo
    {
        let _s: String = String::from("String propriamente dita");
    }
    //    println!("{_s} é inválido aqui fora e o valor apontado por ele já foi excluído"); // <- erro de compilação

    loop {
        let _s: String = String::from("String propriamente dita");
        break;
    }
    //    println!("{_s} é inválido aqui fora e o valor apontado por ele já foi excluído"); // <- erro de compilação

    // -- Reatribuição de Owner
    let mut _real_string: String = String::from("String propriamente dita");
    _real_string = String::from("Valor diferente");
    // O valor "String propriamente dita" apontado anteriormente já foi excluido automaticamente da memória

    //Em outras linguagens ele ficaria esquecido.





// BORROW SITUATIONS

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





    // -- Situação 4 (ERRO) -- Borrow de ownercomo parâmetro para função
    let _real_string: String = String::from("String propriamente dita");

    function(_real_string);

    //    println!("{realString} Foi movida para 's' da função, não existe mais"); // <- erro de compulação (borrow of moved value)

    let _real_string: String = String::from("String propriamente dita");
    println!("{_real_string}");
    println!("{_real_string}");





    // -- Situalção 5 (OK) -- Usando clone()
    let _real_string: String = String::from("String propriamente dita"); // String pode modificar seu tamanho, visto ser um tipo complexo;

    let _x: String = _real_string.clone();

    println!("clone de {_real_string} foi movido para dentro de X ({_x}), realString ainda existe");
}

fn function(s: String) {
    println!("{s}");
}
