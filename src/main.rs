// Learning struc and enum

struct Rectangle{
    width: u32,
    heigth: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.heigth * self.width
    }

    // func having same name as property on rectangle

    fn width(&self) -> u32{
        self.width
    }

    fn heigth(&self) ->u32{
        self.heigth
    }
}

enum Coin {
    Penny,
    Nickel,
    Dim
}

fn value_to_coin(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dim => 10
    }
}

// Defining Options

enum Option<T> {
    None,
    Some(T)
}

fn main() {
    let penny = value_to_coin(Coin::Penny);
    let nickel = value_to_coin(Coin::Nickel);
    let dim = value_to_coin(Coin::Dim);

    println!("value for penny {}", penny);
    println!("value for nickel {}", nickel);
    println!("value for dim {}", dim);
}
