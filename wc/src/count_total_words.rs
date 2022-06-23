use colored::*;
/// Count the total number of words
pub fn count_total_words(vec: &Vec<String>) {
    let mut nvec = Vec::new();
    for i in vec {
        let a = i;
        let s = a.split(" ").collect::<Vec<&str>>();
        for j in s {
            nvec.push(j);
        }
    }
    let cntr = nvec.len();
    println!(
        "{}: {}",
        "Total words in the text".bright_purple().bold(),
        cntr.to_string().bright_red().italic().bold()
    );
}
