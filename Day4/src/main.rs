use std::{fs, collections::HashMap};

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day4\\src\\input.txt",
    )
    .expect("Unable to read file");
    let mut sum = 0;
    let mut copies: HashMap<u32, u32> = HashMap::new();
    for i in 1..=199 {
        copies.insert(i, 1);
    }
    let mut card_num = 1;
    for line in data.lines() {
        let winners: Vec<&str> = line[line.find(":").unwrap() + 1..line.find("|").unwrap() - 1]
            .split(" ")
            .filter(|x| x.ne(&""))
            .collect();

        let mine: Vec<&str> = line[line.find("|").unwrap() + 1..line.len()]
            .split(" ")
            .filter(|x| x.ne(&""))
            .collect();
        //let base: i32 = 2;
        let mut score = 0;
       
        for num in mine {
            if winners.contains(&num) {
                score += 1;
            }
        } 
        for i in 1..=score {
            copies.insert(card_num + i, copies.get(&(&card_num + i)).unwrap() + copies.get(&card_num).unwrap());
        }
        card_num += 1;
        


        //part1
        // if score > 0 {
        //     sum += base.pow(score - 1)
        // }
    }
    for pairs in copies {
        sum += pairs.1;
    }
    println!("{}", sum);
}
