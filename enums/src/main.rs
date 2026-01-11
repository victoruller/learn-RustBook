#![allow(unused)]

/// documentation for IpAddr
struct Ipv6Addr {
    // --snip--
}
enum IpAddr {
    V1(u8),             // Int enum
    V4(u8, u8, u8, u8), // Tuple enum
    V6(Ipv6Addr),       // Struct enum
    // VV(IpAddr),                       <-(Compler error)- "recursive type `main::IpAddr` has infinite size"
}

enum DeEnums {
    T1(IpAddr),         // Enum enum
    // T2(IpAddr::V4)                    <-(Compler error)- "expected type, found variant `IpAddr::V4`"
}

fn main() {
    let home     = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(Ipv6Addr {});

    let none: Option<i32> = Option::None;


    /* --- USAR MATCH COM ENUM --- */
    #[derive(Debug)]
    enum Estado {
        SC,
        PR,
        RS,
    }
    #[derive(Debug)]
    enum Politico {
        Governador(Estado),
        Senador(Estado),
        Deputado(Estado),
        Presidente,
    }
    // let victor = Politico::Governador(Estado::SC);
    let victor = Politico::Deputado(Estado::SC);
    match &victor {
        Politico::Governador(estado) => println!("Governador eleito em {estado:?}"), // <- "estado" é usado como uma variável
        Politico::Senador(estado)    => println!("Senador eleito em {estado:?}"),
        Politico::Presidente         => println!("Presidente eleito"),
        outro => println!("{outro:?} eleito"),                                       // <- "outro" é usado como uma variável
    }
    match &victor {
        Politico::Governador(Estado::SC) => println!("SC"),
        Politico::Governador(Estado::PR) => println!("PR"),
        _ => (),
    }
}
