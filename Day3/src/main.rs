use std::{fs, vec};

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day3\\src\\input.txt",
    )
    .expect("Unable to read file");

    let mut sum = 0;
    let mut arr: Vec<Vec<char>> = Vec::new();
    let mut donezo: Vec<Vec<bool>>;
    let numbers = "1234567890";
    for line in data.lines() {
        arr.push(line.chars().collect::<Vec<_>>())
    }
    donezo = vec![vec![false; arr.len()]; arr[0].len()];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j].eq(&'*') {
                let part_numbers = sweep(j, i, arr.clone());
                    let mut meow: Vec<u32> = Vec::new();
                    for pair in part_numbers {
                         meow.push(extract(pair.0, pair.1, arr.clone(), &mut donezo)
                            .parse::<u32>()
                            .unwrap());
                    }
                    println!("{:?}", meow);
                    let blah: Vec<u32> = meow.into_iter().filter(|x| *x != 0).collect();
                    if blah.len() == 2 {
                        sum += blah[0] * blah[1]
                    }
                   
            }
        }
    }
    println!("{}", sum)

    //part 1
    // donezo = vec![vec![false; arr.len()]; arr[0].len()];
    // for i in 0..arr.len() {
    //     for j in 0..arr[i].len() {
    //         if arr[i][j].ne(&'.') && !numbers.contains(arr[i][j]) {
    //             let part_numbers = sweep(j, i, arr.clone());
    //             for pair in part_numbers {
    //                 sum += extract(pair.0, pair.1, arr.clone(), &mut donezo)
    //                     .parse::<u32>()
    //                     .unwrap();
    //             }
    //         }
    //     }
    // }
    // println!("{}", sum)
}

fn extract(
    row: usize,
    mut index: usize,
    arr: Vec<Vec<char>>,
    donezo: &mut Vec<Vec<bool>>,
) -> String {
    if donezo[index][row] == true {
        return String::from("0");
    }
    let numbers = "1234567890";
    let mut ret = String::from("");
    let r = &arr[row];
    let mut start = 0;
    if index == 0 {
        start = 0;
    } else {
        while index >= 1 && numbers.contains(r[index - 1]) {
            //println!{"{}", r[index-1]}
            index -= 1;
        }
        start = index;
    }
    while index + 1 < r.len() && numbers.contains(r[index + 1]) {
        //println!{"{}", r[index+1]}
        index += 1;
    }
    //println!("{},{}", start, index);
    for i in start..=index {
        donezo[i][row] = true;
        ret.push(r[i]);
    }
    //println!{"{}", ret};
    ret
}

fn sweep(row: usize, col: usize, arr: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut part_numbers = Vec::new();

    if row != 0 {
        //bot left
        if col != 0 {
            if row - 1 >= 0 && col - 1 >= 0 {
                let beep = arr[col - 1][row - 1];
                if beep.is_ascii_digit() {
                    part_numbers.push((col - 1, row - 1))
                }
            }
        }
        if row - 1 >= 0 {
            //left mid
            let beep = arr[col][row - 1];
            if beep.is_ascii_digit() {
                part_numbers.push((col, row - 1))
            }
        }
        //  if col + 1 < arr.len() {
        //top left
        if row - 1 >= 0 {
            let beep = arr[col + 1][row - 1];
            if beep.is_ascii_digit() {
                part_numbers.push((col + 1, row - 1))
            }
        }
        let beep = arr[col + 1][row];
        if beep.is_ascii_digit() {
            part_numbers.push((col + 1, row))
        }
        //  if row + 1 < arr[0].len() {

        //bottom right
        let beep = arr[col + 1][row + 1];
        if beep.is_ascii_digit() {
            part_numbers.push((col + 1, row + 1))
        }
        //  }
        //    }
    }

    if col != 0 {
        if col - 1 >= 0 {
            let beep = arr[col - 1][row]; //top mid
            if beep.is_ascii_digit() {
                part_numbers.push((col - 1, row))
            }
        }
        //  if row + 1 < arr[0].len() {
        if col - 1 >= 0 {
            // top right
            let beep = arr[col - 1][row + 1];
            if beep.is_ascii_digit() {
                part_numbers.push((col - 1, row + 1))
            }
        }
        // }
    }
    // if row + 1 < arr[0].len() {

    let beep = arr[col][row + 1]; //mid right
    if beep.is_ascii_digit() {
        part_numbers.push((col, row + 1))
    }

    //  }
    part_numbers
}
