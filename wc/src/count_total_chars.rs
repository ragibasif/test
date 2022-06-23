use colored::*;
/// Count the total number of chars
pub fn count_total_chars(vec: &Vec<String>) {
    let mut cntr = 0;
    for i in vec {
        cntr += i.chars().count();
    }
    println!(
        "{}: {}",
        "Total characters in the text".magenta().bold(),
        cntr.to_string().yellow().italic().bold()
    );
}
