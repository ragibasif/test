use colored::*;
/// Count the total number of lines
pub fn count_total_lines(vec: &Vec<String>) {
    let len = vec.len();
    println!(
        "{}: {}",
        "Total lines of text".blue().bold(),
        len.to_string().cyan().italic().bold()
    );
}
