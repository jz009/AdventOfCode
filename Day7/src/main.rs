use std::cmp::Ordering;
use std::fs;

static card_ranks: &str = "AKQT98765432J";

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day7\\src\\input.txt",
    )
    .expect("Unable to read file");

    let mut input: Vec<(Hand, u32)> = vec![];
    for line in data.lines() {
        let blah: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut iter = blah[0].chars();
        input.push((Hand {cards: vec![Card {value: iter.next().unwrap()}, Card {value: iter.next().unwrap()}, Card {value: iter.next().unwrap()}, Card {value: iter.next().unwrap()}, Card {value: iter.next().unwrap()}]}, blah[1].parse::<u32>().unwrap()))
    }

    input.sort_by(|a, b| a.0.cmp(&b.0));
    //println!("{:?}", input);
    let mut winnings = 0;
    let mut index = 1;
    for h in input {
        print!("{:?}", h.0.cards[0].value);
        print!("{:?}", h.0.cards[1].value);
        print!("{:?}", h.0.cards[2].value);
        print!("{:?}", h.0.cards[3].value);
        print!("{:?}", h.0.cards[4].value);
        println!();
        winnings += index * h.1;
        index += 1;
    }
    println!("{}", winnings);
}

#[derive(Eq)]
#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Eq)]
#[derive(Debug)]
struct Card {
    value: char,
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        card_ranks
            .find(self.value)
            .unwrap()
            .cmp(&card_ranks.find(other.value).unwrap())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        card_ranks
            .find(self.value)
            .unwrap()
            .partial_cmp(&card_ranks.find(other.value).unwrap())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.value == other.value
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let self_score = get_score(self);
        let other_score = get_score(other);
        if self_score.0 == other_score.0 {
            if self_score.1[0] != other_score.1[0] {
                return self_score.1[0].cmp(&other_score.1[0]);
            }
            if self_score.1[1] != other_score.1[1] {
                return self_score.1[1].cmp(&other_score.1[1]);
            }
            if self_score.1[2] != other_score.1[2] {
                return self_score.1[2].cmp(&other_score.1[2]);
            }
            if self_score.1[3] != other_score.1[3] {
                return self_score.1[3].cmp(&other_score.1[3]);
            }
            if self_score.1[4] != other_score.1[4] {
                return self_score.1[4].cmp(&other_score.1[4]);
            }
        }
        get_score(self).0.cmp(&get_score(other).0)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cards[0] == other.cards[0] && self.cards[1] == other.cards[1] && self.cards[2] == other.cards[2] && self.cards[3] == other.cards[3] && self.cards[0] == other.cards[0]
    }
}

fn get_score(h: &Hand) -> (u32, Vec<u32>) {
    let mut counts: Vec<u32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut woop: Vec<u32> = vec![];
    for card in &h.cards {
        match card.value {
            'J' => {counts[0] += 1; woop.push(1)},
            '2' => {counts[1] += 1; woop.push(2)},
            '3' => {counts[2] += 1; woop.push(3)},
            '4' => {counts[3] += 1; woop.push(4)},
            '5' => {counts[4] += 1; woop.push(5)},
            '6' => {counts[5] += 1; woop.push(6)},
            '7' => {counts[6] += 1; woop.push(7)},
            '8' => {counts[7] += 1; woop.push(8)},
            '9' => {counts[8] += 1; woop.push(9)},
            'T' => {counts[9] += 1; woop.push(10)},
            'Q' => {counts[10] += 1; woop.push(12)},
            'K' => {counts[11] += 1; woop.push(13)},
            'A' => {counts[12] += 1; woop.push(14)},
            _ => {}
        }
    }
    let mut index = 1;
    let num_jokers = counts[0];
    for count in &mut counts[1..] {
        if *count == 5 {
            return (7 + 12, woop);
        }
        if *count == 4 {
            if num_jokers == 1 {
                return (7 + 12, woop)
            }
            return (6 + 12, woop);
        }
        if *count == 3 {
            let mut is_two = false;
            for count2 in &mut counts[1..] {
                if *count2 == 2 {
                    is_two = true;
                }
            }
            if is_two {
                return (5 + 12, woop);
            }
            if num_jokers == 1 {
                return (6 + 12, woop)
            }
            if num_jokers == 2 {
                return (7 + 12, woop)
            }
            return (4 + 12, woop);
        }
        if *count == 2 {
            
            let mut is_three = false;
            let mut is_two = false;
            for count2 in &mut counts[index+1..] {
                if *count2 == 3 {
                    is_three = true;
                }
                if *count2 == 2 {
                    is_two = true;
                }
            }
            if is_three {
                return (5 + 12, woop)
            }
            if is_two {
                if num_jokers == 1 {
                    return (5 + 12, woop)
                }
                return (3 + 12, woop);
            }
            if num_jokers == 1 {
                return (4 + 12, woop)
            }
            if num_jokers == 2 {
                return (6 + 12, woop)
            }
            if num_jokers == 3 {
                return (7 + 12, woop)
            }
            return (2 + 12, woop);
        }
        index += 1;
    }
    if num_jokers == 1 {
        return (2 + 12, woop)
    }
    if num_jokers == 2 {
        return (4 + 12, woop)
    }
    if num_jokers == 3 {
        return (6 + 12, woop)
    }
    if num_jokers == 4 {
        return (7 + 12, woop)
    }
    if num_jokers == 5 {
        return (7 + 12, woop)
    }
    
    (1, woop)
}
