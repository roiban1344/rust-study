use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<String, i32> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let score = match scores.get(&team_name) {
        Some(score) => score,
        None => {
            println!("{} not exist", team_name);
            return;
        }
    };
    println!("{}", score);

    scores.insert(String::from("Green"), 300);
    scores.insert(String::from("Red"), 70);
    scores.entry(String::from("Red")).or_insert(60);
    scores.entry(String::from("Purple")).or_insert(90);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
}
