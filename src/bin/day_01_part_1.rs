fn calibration_value(line: &str) -> u32 {
    // Filter just the digits in `line`
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap();
    let last = digits.next_back().unwrap_or(first);
    10 * first + last
}

fn main() {
    // Read the input file "day_01_test.txt"
    // and store it in the variable "input"
    let input = include_str!("../inputs/day_01.txt");
    let lines = input.lines();

    let result = lines.map(calibration_value).sum::<u32>();

    println!("Result: {}", result);
}
