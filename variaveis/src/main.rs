fn main() {


//VARIÁVEIS
    let _x = 5; // <- Ok
    let _x: i32 = 7; // <- Ok
//    x = 6; // <- Erro de compilação (x é imutável)
    let mut _x = '6'; // <- Ok (shadowing)
    let mut _x = 6; // <- Ok (shadowing)
    _x = 7; // <- Ok (x é mutável)

    let _x = if _x == 5 { 5 } else { 6 }; // <- Ok (já que if é uma expressão)
//    let x = if x == 5 { 5 } else { "6" }; // <- Erro de compilação (tipos diferentes)


//CONSTANTES
//    const _X = 5; // <- Erro de compilação (não é possível declarar uma constante sem tipo)
    const _Y: i32 = 5; // <- Ok
//    _Y = 6; // <- Erro de compilação (Y é imutável)
//    const _Y: i32 = 5; // <- Erro de compilação (shadowing)
//    const _Z: i32 = _x; // <- Erro de compilação (non-constant value), não é possível atribuir uma variável a uma constante
    const _Z: i32 = 5 + 5; // <- Ok
    const _ZZ: i32 = _Y; // <- Ok
    const _ZZZ: i32 = if _Z == 5 { 5 } else { 6 }; // <- Ok (já que if é uma expressão)
//    const ZZZZ: i32 = if x == 5 { 5 } else { 6 }; // <- Erro de compilação (non-constant value), não é permitido ter uma variável na declaração da constante



//TUPLAS
    let _tupla = (1, 2, 3);
//    _tupla.0 = 4; // <- Erro de compilação (tupla é imutável)
    
    let mut _tupla = (1, 2, 3, 4); // <- Ok (shadowing)
    _tupla.0 = 4; // <- Ok (tupla é mutável)
//    _tupla = (1, 2, 3, 4, 5); // <- Erro de compilação (tupla não pode mudar de tamanho)
    let (_a, _b, _c, _d) = _tupla; // <- Ok (destructuring)
//    let (a, b, c, d, e) = _tupla; // <- Erro de compilação (número de elementos diferente)
//    let (a, b, c) = _tupla; // <- Erro de compilação (número de elementos diferente)


//ARRAYS
    let _array = [1, 2, 3, 4, 5];
//    array[0] = 6; // <- Erro de compilação (array é imutável)

    let mut _array = [1, 2, 3, 4, 5]; // <- Ok (shadowing)
    _array[0] = 6; // <- Ok (array é mutável)

    let mut _array2 = [1; 6]; // <- Ok (array de 6 elementos, todos com valor 1)
    _array[1] = 7; // <- Ok (array é mutável)
    
//    array = [1;7]; // <- Erro de compilação (array não pode mudar de tamanho)
    let _array2 = _array; // <- Ok (cópia por valor)

    let [_a, _b, _c, _d, _e] = _array; // <- Ok (destructuring)
//    let [a, b, c, d, e, f] = array; // <- Erro de compilação (número de elementos diferente)





}