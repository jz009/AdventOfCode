use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day12\\src\\input.txt",
    )
    .expect("Unable to read file");
    let mut sum = 0;
    for line in data.lines() {
        let mut iter = line.split(" ");
        let groups = iter.next().unwrap();
        let counts = iter.next().unwrap();
        let groups = vec![groups; 5].join("?");
        let counts = vec![counts; 5].join(",");
        let groups = groups
            .split(".")
            .filter(|x| !x.is_empty())
            .map(|x| x.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let counts = counts
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut memo = HashMap::new();
        sum += inner(0, &groups, 0, &counts, &mut memo);
    }
    dbg!(sum);
}

fn inner(
    group_idx: usize,
    groups: &[Vec<u8>],
    count_idx: usize,
    counts: &[u64],
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if memo.contains_key(&(group_idx, count_idx)) {
        return memo[&(group_idx, count_idx)];
    }
    if group_idx >= groups.len() && count_idx >= counts.len() {
        memo.insert((group_idx, count_idx), 1);
        return 1;
    }
    if group_idx >= groups.len() {
        memo.insert((group_idx, count_idx), 0);
        return 0;
    }
    if count_idx >= counts.len() {
        if groups[group_idx..].iter().flatten().all(|x| *x == b'?') {
            memo.insert((group_idx, count_idx), 1);
            return 1;
        }
        memo.insert((group_idx, count_idx), 0);
        return 0;
    }

    let mut permutations: HashMap<Vec<u8>, u64> = HashMap::new();
    permutations.insert(vec![], 1);
    let group = &groups[group_idx];
    for c in group {
        if *c == b'#' {
            permutations = permutations
                .into_iter()
                .map(|(mut k, v)| {
                    k.push(b'#');
                    (k, v)
                })
                .filter(|(k, _)| {
                    let run_lengths = get_run_lengths(k);
                    count_idx + run_lengths.len() - 1 < counts.len()
                        && run_lengths.last().unwrap() <= &counts[count_idx + run_lengths.len() - 1]
                })
                .collect();
        } else {
            let mut c: HashMap<Vec<u8>, u64> = HashMap::new();
            for (p, perm_counts) in permutations {
                if p.is_empty() {
                    *c.entry(vec![]).or_default() += perm_counts;
                    *c.entry(vec![b'#']).or_default() += perm_counts;
                    continue;
                }
                let run_lengths = get_run_lengths(&p);
                
                if run_lengths.len() == counts.len() - count_idx && run_lengths[run_lengths.len() -1] == counts[counts.len() - 1] {
                    *c.entry(p.clone()).or_default() += perm_counts;
                }
                else if p[p.len() - 1] == b'#' && run_lengths[run_lengths.len() -1] == counts[count_idx + run_lengths.len() - 1] {
                    let mut something = p.clone();
                    something.push(b'.');
                    *c.entry(something).or_default() += perm_counts;
                } else {
                    if p[p.len() - 1] == b'.' {
                        *c.entry(p.clone()).or_default() += perm_counts;
                    }
                    let mut something = p.clone();
                    something.push(b'#');
                    *c.entry(something).or_default() += perm_counts;
                }
            }
            permutations = c;
        }
    }
    let mut perm_counts: HashMap<Vec<u8>, u64> = HashMap::new();
    for (mut k, v) in permutations {
        if k.last() == Some(&b'.') {
            k.pop();
        }
        *perm_counts.entry(k).or_default() += v;
    }
    let mut num_permutations: u64 = 0;
    for (p, perm_count) in perm_counts {
        let run_lengths = get_run_lengths(&p);
        let matches = run_lengths
            .iter()
            .enumerate()
            .all(|(i, len)| count_idx + i < counts.len() && counts[count_idx + i] == *len);
        if matches {
            num_permutations += perm_count
                * inner(
                    group_idx + 1,
                    groups,
                    count_idx + run_lengths.len(),
                    counts,
                    memo,
                );
        }
    }
    memo.insert((group_idx, count_idx), num_permutations);
    num_permutations
}

fn get_run_lengths(slice: &[u8]) -> Vec<u64> {
    let mut run = 0;
    let mut run_lengths = vec![];
    for n in slice {
        if *n == b'.' {
            if run == 0 {
                continue;
            }
            run_lengths.push(run);
            run = 0;
            continue;
        }
        run += 1;
    }
    if run > 0 {
        run_lengths.push(run);
    }
    run_lengths
}
