/// Turn the contents of the file to vector by line
pub fn turn_to_vec(contents: &String) -> Vec<String> {
    let mut vec = Vec::new();
    let data = contents.split("\n").collect::<Vec<&str>>();
    for line in data {
        if line == "" {
            continue;
        }
        vec.push(line.to_string());
    }
    vec
}
