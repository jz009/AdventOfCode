use std::fs;

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day1\\src\\input.txt",
    )
    .expect("Unable to read file");
    let mut sum = 0;

    for line in data.lines() {
        let mut first = 0;
        let mut second = 0;
        for i in 0..line.len()+1 {
            let slice = &line[0..i];
            if vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]
                .iter()
                .any(|target| slice.contains(target))
            {
                let t: String = slice.chars().filter(|c| c.is_ascii_digit()).collect();
                first = t.parse::<u8>().unwrap();
                break;
            } else if vec![
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .any(|target| slice.contains(target))
            {
                if slice.contains("one") {
                    first = 1;
                } else if slice.contains("two") {
                    first = 2
                } else if slice.contains("three") {
                    first = 3
                } else if slice.contains("four") {
                    first = 4
                } else if slice.contains("five") {
                    first = 5
                } else if slice.contains("six") {
                    first = 6
                } else if slice.contains("seven") {
                    first = 7
                } else if slice.contains("eight") {
                    first = 8
                } else {
                    first = 9
                }
                break;
            }
        }
        for i in (0..line.len()).rev() {
            let slice = &line[i..(line.len())];
            if vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]
                .iter()
                .any(|target| slice.contains(target))
            {
                let t: String = slice.chars().filter(|c| c.is_ascii_digit()).collect();
                second = t.parse::<u8>().unwrap();
                break;
            } else if vec![
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .any(|target| slice.contains(target))
            {
                if slice.contains("one") {
                    second = 1;
                } else if slice.contains("two") {
                    second = 2
                } else if slice.contains("three") {
                    second = 3
                } else if slice.contains("four") {
                    second = 4
                } else if slice.contains("five") {
                    second = 5
                } else if slice.contains("six") {
                    second = 6
                } else if slice.contains("seven") {
                    second = 7
                } else if slice.contains("eight") {
                    second = 8
                } else {
                    second = 9
                }
                break;
            }
        }
        let s = format!("{}{}", first, second);
        let num = s.parse::<i32>().unwrap();
        sum += num;
    }
    println!("{}", sum)
}
