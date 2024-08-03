fn to_digit(s: &str) -> Option<u32> {
    match s {
        s if s.starts_with("one") => Some(1),
        s if s.starts_with("two") => Some(2),
        s if s.starts_with("three") => Some(3),
        s if s.starts_with("four") => Some(4),
        s if s.starts_with("five") => Some(5),
        s if s.starts_with("six") => Some(6),
        s if s.starts_with("seven") => Some(7),
        s if s.starts_with("eight") => Some(8),
        s if s.starts_with("nine") => Some(9),
        s => s.chars().next().and_then(|c| c.to_digit(10)),
    }
}

fn get_digits(line: &str) -> impl DoubleEndedIterator<Item = u32> + '_ {
    // Generate an iterator of overlapping windows starting at each character in `line`
    let windows = line.char_indices().map(|(i, _)| &line[i..]);
    windows.filter_map(to_digit)
}

fn calibration_value(line: &str) -> u32 {
    let mut digits = get_digits(line);
    let first = digits.next().unwrap();
    let last = digits.next_back().unwrap_or(first);
    10 * first + last
}

fn main() {
    let input = include_str!("../inputs/day_01.txt");
    let lines = input.lines();

    let result = lines.map(calibration_value).sum::<u32>();

    println!("Result: {result}");
}
