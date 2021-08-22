fn to_pig_latin(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    match chars.first() {
        Some(&c) => match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                format!("{}hay", word)
            }
            'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r'
            | 's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => {
                let mut word = String::new();
                for &d in &chars[1..] {
                    word = word + &d.to_string()
                }
                word + &c.to_string() + "ay"
            }
            _ => panic!(),
        },
        None => String::from(""),
    }
}

fn main() {
    assert_eq!(&to_pig_latin("first"), "irstfay");
    assert_eq!(&to_pig_latin("apple"), "applehay");
}
