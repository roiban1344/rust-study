fn main() {
    let s = String::new();
    
    println!("{} {}", s, s.len());
    
    let mut hello = "hello".to_string();
    println!("{}", hello);
    hello.push_str(", string!");
    println!("{}", hello);

    let foo  = String::from("foo");
    let bar = String::from("bar");
    let foo_bar_baz = foo + &bar + "baz";
    //println!("{}", foo);
    println!("{}", bar);
    println!("{}", foo_bar_baz);

    let s0 = String::from("tic");
    let s1 = String::from("tac");
    let s2 = String::from("toe");
    let s = format!("{}-{}-{}", s0, s1, s2);

    println!("{}", s0);
    println!("{}", s1);    
    println!("{}", s2);
    println!("{}", s);

    let en = String::from("Hello");
    let ru = String::from("Здравствуйте");
    let jp = String::from("こんにちは");
    println!("{} {}", en, en.len());
    println!("{} {}", ru, ru.len());
    println!("{} {}", jp, jp.len());

    println!("[..6] {}", &jp[..6]);
    //println!("[..7] {}", &jp[..7]);

    let emoji = String::from("👩🏻‍💻");
    for c in emoji.chars(){
        println!("{}", c);
    }

    for b in emoji.bytes(){
        println!("{}", b);
    }
}
