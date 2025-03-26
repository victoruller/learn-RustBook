fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
        break 5;
    };

    let _x: i32 = result;

    println!("The result is {result}");


    let _x = 'primeiro: loop {
        'segundo: loop {
            'terceiro: loop {
                break 'primeiro 1;
                continue 'segundo;
            };
        };
    };


    /*let x = loop {
        if x = 1 {
            break 1;
        } else if x = 2 {
            break 'h';
        }
    }; <- Erro de compilação*/





    let mut counter = 0;
    let _a =  'primeiro: while counter < 5 {
        println!("counter: {}", counter);


        loop {
            println!("counter: {}", counter);
            counter += 1;
            break 'primeiro;
        };


        counter += 1;
        if counter == 3 {
            break;
        }
    };


    'primeiro: for a in 1..10 {
        'segundo: for b in 1..10 {
            if a == 2 {
                continue;
            }
    
            println!("for: {}", a);
    
            if a == 4 { break 'primeiro; }
        };
    };


}