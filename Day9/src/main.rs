// use std::fs;

// fn main() {
//     let data = fs::read_to_string(
//         "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day9\\src\\input.txt",
//     )
//     .expect("Unable to read file");
//     let mut sum = 0;
//     for line in data.lines() {
//         let nums: Vec<i32> = line.split_ascii_whitespace().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
//         sum += recurse(&nums);
//     }
//     println!("{}", sum);
// }

// fn recurse(row: &[i32]) -> i32 {
//     if row.iter().all(|n| *n == row[0]) {
//         return row[row.len() - 1];
//     }

//     let mut next = Vec::new();

//     for i in 0..row.len()-1 {
//         next.push(row[i+1] - row[i]);
//     }

//     return row[row.len() - 1] + recurse(&next);
// }


use std::fs;

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day9\\src\\input.txt",
    )
    .expect("Unable to read file");
    let mut sum = 0;
    for line in data.lines() {
        let nums: Vec<i32> = line.split_ascii_whitespace().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        sum += recurse(&nums);
    }
    println!("{}", sum);
}

fn recurse(row: &[i32]) -> i32 {
    if row.iter().all(|n| *n == row[0]) {
        return row[0];
    }

    let mut next = Vec::new();

    for i in 0..row.len()-1 {
        next.push(row[i+1] - row[i]);
    }

    return row[0] - recurse(&next);
}