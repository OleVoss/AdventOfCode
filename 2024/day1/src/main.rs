use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Wrong filepath!");
    let (mut loc1, mut loc2): (Vec<i32>, Vec<i32>) = contents
        .split("\n")
        .map(|line| {
            let mut iter = line.split_whitespace();
            let n1 = iter.next().unwrap().parse::<i32>().unwrap();
            let n2 = iter.next().unwrap().parse::<i32>().unwrap();
            return (n1, n2);
        })
        .unzip();

    loc1.sort();
    loc2.sort();

    let pairs: Vec<(i32, i32)> = loc1
        .clone()
        .into_iter()
        .zip(loc2.clone().into_iter())
        .collect();
    let sum: i32 = pairs.into_iter().map(|p| (p.0 - p.1).abs()).sum();
    println!("Sum of diffs: {sum}");

    let sim_score: i32 = loc1
        .into_iter()
        .map(|id| {
            let count = loc2.iter().filter(|p| **p == id).count();
            return count as i32 * id;
        })
        .sum();
    println!("Sim Score: {sim_score}");
}
