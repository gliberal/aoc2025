use ferris_says::say;
use std::io::{stdout, BufRead, BufReader, BufWriter, Error};
use std::fs::File;

fn main() -> Result<(), Error> {
    println!("Day 01");

    let start_dial_value = 50;
    let mut current_dial_value = start_dial_value;
    let mut zero_cpt = 0;

    // Current dir :
    println!("Current dir: {:?}", std::env::current_dir().unwrap());

    // Iterate over file lines
    let input_file_path = "input.txt";
    let file_content = File::open(input_file_path)?;
    let file_reader = BufReader::new(file_content);

    for line_result in file_reader.lines() {
        let line = line_result?;
        let line = line.trim();  // Remove leading and trailing whitespace

        // split line into letter & number
        let letter = line.chars().next().unwrap();
        let number:i32 = line[1..].parse().unwrap();

        let mut tmp_result:i32;
        if letter == 'L' {
            tmp_result = current_dial_value - number;

            while tmp_result < 0 {
                tmp_result = 100 - tmp_result.abs();
            }
        } else {
            tmp_result = current_dial_value + number;

            while tmp_result >= 100 {
                tmp_result = tmp_result - 100;
            }
        }

        current_dial_value = tmp_result;
        println!("The dial is rotated {} to point at {}", line, current_dial_value);

        if current_dial_value == 0 {
            zero_cpt += 1;
        }
    }

    // Displays the result
    let stdout = stdout();
    let message = format!("{} {}", "Result is ", zero_cpt);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer)?;

    Ok(())
}
