use std::fs;

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day10\\src\\input.txt",
    )
    .expect("Unable to read file");

    let mut grid: Vec<Vec<String>> = vec![];
    let mut pos: (usize, usize) = (0, 0);
    let mut y = 0;
    for line in data.lines() {
        if line.contains('S') {
            pos = (y, line.find("S").unwrap());
        }
        grid.push(line.chars().map(|x| x.to_string()).collect());
        y += 1;
    }
    let north = vec![
        String::from("S"),
        String::from("|"),
        String::from("7"),
        String::from("F"),
    ];
    let south = vec![
        String::from("S"),
        String::from("|"),
        String::from("L"),
        String::from("J"),
    ];
    let east = vec![
        String::from("S"),
        String::from("-"),
        String::from("L"),
        String::from("F"),
    ];
    let west = vec![
        String::from("S"),
        String::from("-"),
        String::from("J"),
        String::from("7"),
    ];
    let mut value = 0;
    loop {
        let cur_pos = (pos.0, pos.1);
        //south
        if pos.0 != 0
            && south.contains(&grid[pos.0][pos.1])
            && north.contains(&grid[pos.0 - 1][pos.1])
        {
            if grid[pos.0][pos.1] == "J"
                || grid[pos.0][pos.1] == "F"
            {
                grid[pos.0][pos.1] = "%".to_string();
            } else {
                grid[pos.0][pos.1] = "$".to_string();
            }
            value += 1;
            pos.0 -= 1;
        }
        //north
        else if pos.0 != grid[0].len() - 1
            && north.contains(&grid[pos.0][pos.1])
            && south.contains(&grid[pos.0 + 1][pos.1])
        {
                if grid[pos.0][pos.1] == "J"
                    || grid[pos.0][pos.1] == "F"
                {
                    grid[pos.0][pos.1] = "%".to_string();
                } else {
                    grid[pos.0][pos.1] = "$".to_string();
                }
            value += 1;
            pos.0 += 1;
        }
        //west
        else if pos.1 != 0
            && west.contains(&grid[pos.0][pos.1])
            && east.contains(&grid[pos.0][pos.1 - 1])
        {
                if grid[pos.0][pos.1] == "J"
                    || grid[pos.0][pos.1] == "F"
                {
                    grid[pos.0][pos.1] = "%".to_string();
                } else {
                    grid[pos.0][pos.1] = "$".to_string();
                }
            value += 1;
            pos.1 -= 1;
        }
        //east
        else if pos.1 != grid.len() - 1
            && east.contains(&grid[pos.0][pos.1])
            && west.contains(&grid[pos.0][pos.1 + 1])
        {
                if grid[pos.0][pos.1] == "J"
                    || grid[pos.0][pos.1] == "F"
                {
                    grid[pos.0][pos.1] = "%".to_string();
                } else {
                    grid[pos.0][pos.1] = "$".to_string();
                }
            value += 1;
            pos.1 += 1;
        }
        if cur_pos.0 == pos.0 && cur_pos.1 == pos.1 {
            if grid[pos.0][pos.1] == "J"
                    || grid[pos.0][pos.1] == "F"
                {
                    grid[pos.0][pos.1] = "%".to_string();
                } else {
                    grid[pos.0][pos.1] = "$".to_string();
                }
            break;
        }
    }

    let mut count = 0;
    for col in 0..grid.len() {
        for row in 0..grid[0].len() {
            let mut row2 = row;
            let mut col2 = col;
            let mut inter = 0;
            loop {
                if grid[col][row] == "%" || grid[col][row] == "$" {
                    break;
                }
                if grid[col2][row2] == "$" {
                    inter += 1;
                }
                if grid[col2][row2] == "%" {
                    inter += 2;
                }
                if row2 != 0 {
                    row2 -= 1;
                } else {
                    break;
                }
                if col2 != grid.len() - 1 {
                    col2 += 1;
                } else {
                    break;
                }
            }
            if inter == 0 {
                continue;
            }
            if inter % 2 == 1 {
                grid[col][row] = "*".to_string();
                count += 1;
            }
        } 
    }

    for line in grid {
        println!("{:?}", line)
    }
    println!("{:?}", count);
}
