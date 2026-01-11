
#[derive(Debug)] // <- implementa automaticamente as traits de Debug
struct Rectangle {
    width:  u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect_1 = dbg!(
        Rectangle {
            width:  10,
            height: 45,
        }
    ); // <- dbg! retorna o objeto

    println!("The rectangle is: {rect_1:?}");  // <- ':?' = debug output format
    println!("The rectangle is: {rect_1:#?}"); // <- ':#?' = debug output format breaking lines
    dbg!(&rect_1);                             // <- dbg!() mostra igual ao ':#?', mas no stderr

    // using struct as parameter
    println!(
        "1. the area of the rectangle is: {} square pixels -> {} x {}",
        area(&rect_1), rect_1.width, rect_1.height
    );
    
    // using struct method
    println!(
        "2. the area of the rectangle is: {} square pixels -> {} x {}",
        rect_1.area(), rect_1.width, rect_1.height
    );

    // using pointer to struct
    let pointer = &rect_1;
    println!(
        "3. the area of the rectangle is: {} square pixels -> {} x {}",
        pointer.area(), pointer.width, pointer.height
    );

    // using struct method generically (associated functions)
    let square = Rectangle::square(30);
    println!(
        "4. the area of the square is: {} square pixels -> {} x {}",
        square.area(), square.width, square.height
    );

    // using method as associated function
    println!(
        "5. the area of the square is: {}",
        Rectangle::area(&square)
    )
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}