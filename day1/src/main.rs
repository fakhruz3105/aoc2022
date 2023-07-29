use std::fs;

fn main() {
    let file = fs::read_to_string("index.txt").unwrap();
    let elfs: Vec<&str> = file.split("\n\n").collect();

    let mut elfs_cals: Vec<u32> = elfs
        .iter()
        .map(|elf| {
            let calories: Vec<u32> = elf.split("\n").map(|a| a.parse().unwrap_or(0)).collect();
            calories.iter().sum()
        })
        .collect();

    elfs_cals.sort_by(|a, b| b.cmp(a));
    let top_three_total: u32 = elfs_cals[0..3].iter().sum();
    println!("{}", top_three_total);
}
