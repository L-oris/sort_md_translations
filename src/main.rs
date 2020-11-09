use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_content = fs::read_to_string("sample.md")?;

    let grouped_by_first_letter: HashMap<char, Vec<String>> = raw_content
        .lines()
        .filter(|&line| match line {
            line if line.is_empty() => false,
            line if line.starts_with("#") => false,
            line if line.starts_with("---") => false,
            _ => true,
        })
        .map(|line| line.to_lowercase())
        .fold(HashMap::new(), |mut acc, line| {
            let first_char = remove_to(&line).chars().next().unwrap();
            acc.entry(first_char).or_insert(Vec::new()).push(line);
            acc
        });

    println!("{:#?}", grouped_by_first_letter);

    Ok(())
}

// TODO LORIS: impl method on &str
fn remove_to(s: &str) -> &str {
    if s.starts_with("to ") {
        &s[3..]
    } else {
        &s[..]
    }
}

/* OUTLINE:
 *
 * Read <filename.md> (first arg) to String
 * Remove empty lines and those beginning with `#` and `---`
 * Sort lines by customized sorting fn
 * * if line starts with `to `, pick second word
 * * sort alphabetically
 * Group lines by letter
 * Begin each letter-group with `### <letter>`
 * Push `### Mobile` and `### PC` to beginning of String
 * Write to `sorted.md`
 */
