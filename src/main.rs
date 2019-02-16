use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let file = File::open("words_alpha.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // Input
    let dictionary: HashSet<_> = contents.lines().collect();
    let circles: Vec<Vec<char>> = vec![
        "cb".chars().collect(),
        "aoi".chars().collect(),
        "wt".chars().collect(),
    ];

    // Generate words by walking forwards
    let mut words = circles.into_iter().fold(vec!["".to_string()], |words: Vec<String>, chars| {
        chars.into_iter()
            .map(|c| words.clone().into_iter()
                 .map(|mut word| {word.push(c); word})
                 .collect::<Vec<String>>())
            .flatten().collect()
    });

    // Append reversed words (as if by walking backwards through the outter array), if needed
    // words.append(
    //     &mut words.clone().into_iter()
    //     .map(|word| word.chars().rev().collect())
    //     .collect());

    // To have sorted output
    words.sort();

    for word in words.into_iter() {
        if dictionary.contains(word.as_str()) {
            println!("{}", word);
        }
    }

    Ok(())
}
