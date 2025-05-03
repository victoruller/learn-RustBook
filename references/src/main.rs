fn main() {

// REFERENCS ARE IMMUTABLE AS DEFAULT

    let mut s = String::from("hello");
    //let _t = &s;
    //_t.push_str(", w"); // <- (erro de compilação) - not borrowed as mutable
    
    let t = &mut s;
    t.push_str(", world");
    print!("\n{}\n", t); // hello, world
    print!("\n{}\n", s); // hello, world


    //let s = String::from("hello");
    //let t = &mut s; // <- (erro de compilação) - cannot borrow as mutable if its imutable
    //t.push_str(", word");
    //print!("\n{}\n{}\n", s, t);

// A MUTABLE REFERENCE HAS TO BE THE UNIC REFERENCE

    let mut s = String::from("hello");
    let _t1 = &mut s;
  //let _t2 = &mut s; // <- (erro de compilação) - cannot borrow more than one time as mutable
    

/*
    let mut s = String::from("hello");
    let t1 = &s;
    let t2 = &s;
    let t3 = &mut s;
    println!("{} - {} - {}", t1, t2, t3); // <- (erro de compilação) - -Cannot have immutable reference if there is also a mutable one
*/
    
    let mut s = String::from("hello");
    let _t1 = &s;
    let _t2 = &s;
    let t3 = &mut s;
    println!("{}", t3); // <- (OK) references are deleted at their last use


    


}
