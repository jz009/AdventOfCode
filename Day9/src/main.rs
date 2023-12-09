// use std::fs;

// fn main() {
//     let data = fs::read_to_string(
//         "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day9\\src\\input.txt",
//     )
//     .expect("Unable to read file");
//     let mut sum = 0;
//     for line in data.lines() {
//         let nums: Vec<i32> = line.split_ascii_whitespace().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
//         sum += recurse(nums);
//     }
//     println!("{}", sum);
// }

// fn recurse(row: Vec<i32>) -> i32 {
//     println!{"{:?}", row};
//     let mut next: Vec<i32> = vec![];
//     if !is_all_equal(row.clone()) {
//         for i in 0..row.len()-1 {
//            next.push(row[i+1] - row[i]);
//         }
//         next.push(recurse(next.clone()) + row[row.len() -1]);
//         return next[next.len() - 1];
//     }
//     return row[row.len() - 1]
// }

// fn is_all_equal(v: Vec<i32>) -> bool {
//     let comp = v[0];
//     for item in v {
//         if item != comp {
//             return false;
//         }
//     }
//     return true
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
        sum += recurse(nums);
    }
    println!("{}", sum);
}

fn recurse(row: Vec<i32>) -> i32 {
    println!{"{:?}", row};
    let mut next: Vec<i32> = vec![];
    if !is_all_equal(row.clone()) {
        for i in 0..row.len()-1 {
           next.push(row[i+1] - row[i]);
        }
        next.insert(0, row[0] - recurse(next.clone()));
        return next[0];
    }
    return row[0]
}

fn is_all_equal(v: Vec<i32>) -> bool {
    let comp = v[0];
    for item in v {
        if item != comp {
            return false;
        }
    }
    return true
}