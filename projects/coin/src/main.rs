enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin{
    fn value_in_cents(&self) -> u8{
        match self{
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("{}", coin.value_in_cents());
    println!("{}", value_in_cents(coin));
    //println!("{}", Coin::Penny == Coin::Penny);
}
