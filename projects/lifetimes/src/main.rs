fn main() {
    {
        let r;

        //{
            let x = 5;
            r = &x;
        //}

        println!("r: {}", r);
    }

    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(s1.as_str(), s2);
    println!("{}", result);

    let t1 = String::from("foobar");
    let result;
    {
        let t2 = String::from("baz");
        result = longest(t1.as_str(), t2.as_str());
        println!("{}", result);
    }
    //println!("{}", result);

    let t1 = String::from("foobar");
    let t2;
    let result;
    {
        let t3 = String::from("baz");
        t2 = t3; //moved
        result = longest(t1.as_str(), t2.as_str());
    }
    println!("{}", result);

    // let t1 = String::from("foobar");
    // let result;
    // {
    //     let t2 = String::from("baz");
    //     result = longest(t1.as_str(), t2.as_str());
    // }
    // println!("{}", result);

    let novel = String::from("Call me Ishmael/ Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    }else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}