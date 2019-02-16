use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let file = File::open("words_alpha.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let dictionary: HashSet<_> = contents.lines().collect();
    let circles: Vec<Vec<char>> = vec![
        "rlkdt".chars().collect(),
        "slonra".chars().collect(),
        "oeaiu".chars().collect(),
        "bw".chars().collect(),
    ];


    // Generate words by walking from outside the circle
    let mut words = circles.into_iter().fold(vec!["".to_string()], |words: Vec<String>, chars| {
        chars.into_iter()
            .map(|c| words.clone().into_iter()
                 .map(|mut word| {word.push(c); word})
                 .collect::<Vec<String>>())
            .flatten().collect()
    });
    // Append reversed words (as if walking from inside the circle)
    words.append(
        &mut words.clone().into_iter()
        .map(|word| word.chars().rev().collect())
        .collect());
    words.sort();

    for word in words.into_iter() {
        if dictionary.contains(word.as_str()) {
            println!("{}", word);
        }
    }

    Ok(())
}
