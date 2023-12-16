use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day16\\src\\input.txt",
    )
    .expect("Unable to read file");

    let mut grid = vec![];
    for line in data.lines() {
        grid.push(line.as_bytes().to_vec());
    }

    let mut max = 0;
    for x in 0..grid[0].len() {
        let answer = find_path(&grid, x as isize, 0, (0, 1), &mut vec![]);
        let mut set = HashSet::new();
        for a in answer {
            set.insert(a);
        }
        if set.len() > max {
            max = set.len();
        }
    }

    for y in 0..grid.len() {
        let answer = find_path(&grid, 0, y as isize, (1, 0), &mut vec![]);
        let mut set = HashSet::new();
        for a in answer {
            set.insert(a);
        }
        if set.len() > max {
            max = set.len();
        }
    }

    for x in 0..grid[0].len() {
        let answer = find_path(&grid, x as isize, (grid.len() - 1) as isize, (0, -1), &mut vec![]);
        let mut set = HashSet::new();
        for a in answer {
            set.insert(a);
        }
        if set.len() > max {
            max = set.len();
        }
    }

    for y in 0..grid.len() {
        let answer = find_path(&grid, (grid[0].len() - 1) as isize, y as isize, (-1, 0), &mut vec![]);
        let mut set = HashSet::new();
        for a in answer {
            set.insert(a);
        }
        if set.len() > max {
            max = set.len();
        }
    }


    dbg!(max);
}


fn find_path(
    grid: &Vec<Vec<u8>>,
    mut x: isize,
    mut y: isize,
    mut direction: (isize, isize),
    mut been: &mut Vec<(isize, isize, (isize, isize))>,
) -> Vec<(isize, isize)> {
    let mut visited = vec![];
    let left = (-1, 0);
    let right = (1, 0);
    let up = (0, -1);
    let down = (0, 1);

    while x >= 0
        && x < grid[0].len() as isize
        && y  >= 0
        && y  < grid.len() as isize
    {
        if been.contains(&(x, y, direction)) {
            return visited;
        }
        been.push((x, y, direction));
        visited.push((x, y));
        match grid[y as usize][x as usize] {
            b'\\' => {
                if direction == right {
                    direction = down;
                } else if direction == left {
                    direction = up;
                } else if direction == down {
                    direction = right;
                } else {
                    direction = left;
                }
                x += direction.0;
                y += direction.1;
            }
            b'/' => {
                if direction == right {
                    direction = up;
                } else if direction == left {
                    direction = down;
                } else if direction == down {
                    direction = left;
                } else {
                    direction = right;
                }
                x += direction.0;
                y += direction.1;
            }
            b'|' => {
                if direction == left || direction == right {
                    visited.append(&mut find_path(grid, x, y + 1, down, been));
                    visited.append(&mut find_path(grid, x, y - 1, up, been));
                    return visited;
                } else {
                    x += direction.0;
                    y += direction.1;
                }
            }
            b'-' => {
                if direction == up || direction == down {
                    visited.append(&mut find_path(grid, x + 1, y, right, been));
                    visited.append(&mut find_path(grid, x - 1, y, left, been));
                    return visited;
                } else {
                    x += direction.0;
                    y += direction.1;
                }
            }
            _ => {
                x += direction.0;
                y += direction.1;
            }
        }
    }
    visited
}
