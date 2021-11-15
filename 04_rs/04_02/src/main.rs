use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}



fn main() -> io::Result<()> {
    let file = File::open(".\\input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let mut pwords: Vec<&str> = Vec::new();
        let line_unwrap = line.unwrap();
        pwords = line_unwrap.split_whitespace().collect::<Vec<&str>>();

        let mut pwordsSorted: Vec<String> = Vec::new();
        for pword in pwords {
            let mut pword_char: Vec<char> = pword.chars().collect();
            pword_char.sort_unstable();
            let sorted_string: String = pword_char.into_iter().collect();
            pwordsSorted.push(sorted_string);
        }

        if has_unique_elements(pwordsSorted) {
            sum += 1;
        }
    }

    println!("{}", sum);

    Ok(())
}
