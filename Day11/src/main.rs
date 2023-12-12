use std::fs;

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day11\\src\\input.txt",
    )
    .expect("Unable to read file");
    let mut is_doubled_row: Vec<usize> = vec![];
    let mut is_doubled_col: Vec<usize> = vec![];
    let mut index = 0;
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut grid: Vec<Vec<&str>> = vec![];
    let mut sum = 0;
    for line in data.lines() {
        if line.chars().into_iter().all(|n| n == '.') {
            is_doubled_row.push(index);
        }
        grid.push(line.split_terminator("").skip(1).collect::<Vec<_>>());
        index += 1;
    }

    for i in 0..grid[0].len() {
        let mut doubled = true;
        for j in 0..grid.len() {
            if grid[j][i] != String::from(".") {
                doubled = false;
            }
        }
        if doubled {
            is_doubled_col.push(i);
        }
    }
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == String::from("#") {
                galaxies.push((x, y));
            }
        }
    }
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let a = scale(galaxies[i], &is_doubled_row, &is_doubled_col);
            let b = scale(galaxies[j], &is_doubled_row, &is_doubled_col);
            sum += a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        }
    }

    println!("{}", sum);
}

fn scale(
    coords: (usize, usize),
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
) -> (usize, usize) {
    (
        coords.0
            + (1000000 - 1)
                * empty_cols
                    .into_iter()
                    .filter(|c| c <= &&coords.0)
                    .collect::<Vec<_>>()
                    .len(),
        coords.1
            + (1000000 - 1)
                * empty_rows
                    .into_iter()
                    .filter(|r| r <= &&coords.1)
                    .collect::<Vec<_>>()
                    .len(),
    )
}
