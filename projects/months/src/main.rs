use std::io;

fn main() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Please enter a month.");

    let mut month = String::new();

    io::stdin()
        .read_line(&mut month)
        .expect("Failed to read line");

    let month: i32 = month
        .trim()
        .parse()
        .expect("Index entered was not a number");

    match month {
        1..=12 => {
            let index = (month - 1) as usize;
            println!(
                "The name of the {}-th month of a year is: {}",
                month, months[index]
            );
        },
        _ => {
            println!("The {}-th month of a year does not exist.", month);
        },
    };
}
