use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut safe_reports = 0;

    for line in content.lines() {
        let mut levels: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();

        // first bad level
        let up = levels[0] < levels[1];
        let bad_level = levels.windows(2).position(|w| match up {
            true => !((w[0] < w[1]) && ((w[0] - w[1]).abs() < 4)),
            false => !((w[0] > w[1]) && ((w[0] - w[1]).abs() < 4)),
        });
        println!("bad level: {:?}", bad_level);

        match bad_level {
            None => safe_reports += 1,
            Some(bad_level) => {
                levels.remove(bad_level);
                let up = levels[0] < levels[1];
                let bad_level = levels.windows(2).position(|w| match up {
                    true => !((w[0] < w[1]) && ((w[0] - w[1]).abs() < 4)),
                    false => !((w[0] > w[1]) && ((w[0] - w[1]).abs() < 4)),
                });
                match bad_level {
                    Some(_) => (),
                    None => safe_reports += 1,
                }
            }
        }
    }

    println!("{safe_reports} safe reports");
}
