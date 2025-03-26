use std::io; //Define a biblioteca std (standart) como padrão para in and out
use std::cmp::Ordering;//Enumeração que contém Less Greater e Equal
use rand::Rng;

fn main() {
    println!("Guess the number game! hehe");


    let secret_number = rand::rng().random_range(1..=100);//gera um número aleatório entre 1 e
                                                       
    loop{
        println!("Please input yout guess: ");

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        // cria uma instância do tipo 'stdin', que é um handler de input do terminal
        // chama o metodo que cria efetivamente o handler e guarda o valor passado em 'guess'
        // recebe uma enumeração 'return', que pode ser 'ok' ou 'err', expect cracha o programa e
        // exibe uma mensagem se for do tipo err
   
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // retorna num (valor num que está na enumeração)
            Err(_) => continue, // faz a próxima interação de loop
        };
        // Pega a variável guess
       // remove todo espaço em branco do início e final da string (como o \n do enter do usuário) com o trim().
      // transforma a string em unsigned int 32 bit, esse tipo é inferido, pelo compilador, de acordo com o contexo.
     // Cracha se der erro com exepct

    //println!("You guessed: {}, {guess} eh um bom palpite.", guess);


        match guess.cmp(&secret_number) {// esse método retorna se o valor é maior ou menor que o
                                     // parâmetro
       
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Omg, You Wooonnn!");
                break; // esse break quebra nosso loop

            }//match coleta o dado retornado e executa o que for especificado para ele, é como um switch case do C.
        }
    }
}
