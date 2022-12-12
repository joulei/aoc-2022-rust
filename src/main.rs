fn main() {
    let mut calories = std::fs::read_to_string("input/day_one.txt")
        .expect("Input file not found")
        .split("\n\n")
        .map(|calory_lines| {
            calory_lines
                .split('\n')
                .map(|calory| calory.parse().unwrap_or(0u64))
                .sum::<u64>()
        })
        .collect::<Vec<_>>();

    calories.sort_by(|a, b| b.cmp(a));

    // Part one:
    println!("Elf carrying the most calories: {}", calories[0]);

    // Part two:
    println!(
        "Total calories of the top 3 elves: {}",
        calories.iter().take(3).sum::<u64>()
    );
}
