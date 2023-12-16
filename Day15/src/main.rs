use std::fs;

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day15\\src\\input.txt",
    )
    .expect("Unable to read file");
    let blah: Vec<&str> = data.split(",").collect();
    let mut sum = 0;

    let mut my_map = vec![vec![]; 256];
    for b in blah {
        let mut meow; 
        if b.contains('=') {
            meow = b.split("=");
            let burt = meow.next().unwrap();
            let value = bark(burt.as_bytes());
            let poop = meow.next().unwrap().parse::<usize>().unwrap();
            if my_map[value].iter().find(|x: &&(&str, usize)| x.0 == burt).is_some() {
                let index = my_map[value].iter().position(|x| x.0 == burt).unwrap();
                my_map[value][index] = (burt, poop);
            } else {
                my_map[value].push((burt, poop));
            }
        } else {
            meow = b.split("-");
            let can = meow.next().unwrap();
            let hash = bark(can.as_bytes());
            if my_map[hash].iter().find(|x: &&(&str, usize)| x.0 == can).is_some() {
                let index = my_map[hash].iter().position(|x| x.0 == can).unwrap();
                my_map[hash].remove(index);
            }
        }
    }

    let mut box_num = 0;
    for p in my_map {
        let mut tree = 0;
        for n in 0..p.len() {
            tree += (1 + box_num) * (n + 1) * p[n].1;
        }
        box_num += 1;
        sum += tree;
    }
    dbg!(sum);
}


fn bark(bites: &[u8]) -> usize {
    let mut value: usize = 0;
    for c in bites {
        
        value += *c as usize;

        value = (value * 17).into();

        value = value % 256;

    }
    value as usize
}