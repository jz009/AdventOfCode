use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day2\\src\\input.txt",
    )
    .expect("Unable to read file");

    let mut pairs: HashMap<String, i32> = HashMap::new();

    let mut sum = 0;
    for line in data.lines() {
        let pulls = &line[line.find(":").unwrap() + 1..line.len()];
        let blah = &pulls.replace(";", ",");
        let woop: Vec<&str> = blah.split(",").collect();
        pairs.insert(String::from("red"), 0);
        pairs.insert(String::from("green"), 0);
        pairs.insert(String::from("blue"), 0);
        for combo in woop {
            let num: String = combo
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            let color: String = combo
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();
            if pairs.get(&color).unwrap() < &num.parse::<i32>().unwrap() {
                pairs.insert(color, num.parse::<i32>().unwrap());
            }
        }
        let power = pairs.get("red").unwrap() * pairs.get("green").unwrap() * pairs.get("blue").unwrap();
        sum += power;
    }
    println!("{}", sum);
    //part 1
    // let mut pairs = HashMap::new();
    // pairs.insert("red", 12);
    // pairs.insert("green", 13);
    // pairs.insert("blue", 14);
    // let mut sum = 0;
    // let mut possible = true;
    // let mut game = 1;
    // for line in data.lines() {
    //     possible = true;
    //     let pulls = &line[line.find(":").unwrap() + 1..line.len()];
    //     let blah = &pulls.replace(";", ",");
    //     let woop: Vec<&str> = blah.split(",").collect();
    //     for combo in woop {
    //         let num: String = combo
    //             .chars()
    //             .filter(|c| c.is_ascii_digit())
    //             .collect::<String>();
    //         let color: &str = &combo
    //             .chars()
    //             .filter(|c| c.is_alphabetic())
    //             .collect::<String>();
    //         if pairs.get(&color).unwrap() < &num.parse::<i32>().unwrap() {
    //             possible = false;
    //         }
    //     }
    //     if possible {
    //         sum += game;
    //     }
    //     game += 1;
    // }
    // println!("{}", sum)
}
