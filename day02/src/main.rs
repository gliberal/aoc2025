use ferris_says::say;
use std::io::{stdout, BufRead, BufReader, BufWriter, Error};
use std::fs::File;

fn middle_split(s: &str) -> (String, String) {
    let mid = s.chars().count() / 2;
    let first_part: String = s.chars().take(mid).collect();
    let second_part: String = s.chars().skip(mid).collect();
    (first_part, second_part)
}

fn is_repetitive_string(s: &str) -> bool {
    let (first_part, second_part) = middle_split(&s);
    if first_part == second_part {
        return true;
    }
    return false;
}

fn range_filtering(start: u64, end: u64) -> Vec<u64> {
    println!("Read range from {} to {}", start, end);

    let mut invalid_values: Vec<u64> = Vec::new();
    for value in start..=end {
        let value_as_string = value.to_string();

        // Invalid values are repetitive ones, so we check that repetitivity is possible
        if value_as_string.chars().count() % 2 == 0 {
            if is_repetitive_string(&value_as_string) {
                invalid_values.push(value);
            }
        }
    }
    return invalid_values;
}

fn main() -> Result<(), Error> {
    println!("Day 02");

    // Iterate over file lines
    let input_file_path = "input.txt";
    let file_content = File::open(input_file_path)?;
    let file_reader = BufReader::new(file_content);

    let mut invalid_values_sum: u64 = 0;

    // Iterate over file lines
    for line_result in file_reader.lines() {
        let line = line_result?;
        let line = line.trim();  // Remove leading and trailing whitespace

        // split line into ranges
        let product_ranges = line.split(',');

        for product_range in product_ranges {
            let (min_str, max_str) = product_range.split_once('-').unwrap();
            let range_min: u64 = min_str.parse::<u64>().unwrap();
            let range_max: u64 = max_str.parse::<u64>().unwrap();

            // Directly add sum of invalid values
            invalid_values_sum += range_filtering(range_min, range_max).iter().sum::<u64>();
        }
    }

    // Displays the result
    let stdout = stdout();
    let message = format!("{} {}", "Result is ", invalid_values_sum);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer)?;

    Ok(())
}