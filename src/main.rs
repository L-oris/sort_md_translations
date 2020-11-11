use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_content = fs::read_to_string("sample.md")?;

    let mut grouped_by_first_letter: HashMap<char, Vec<String>> = raw_content
        .lines()
        .filter(|&line| match line {
            line if line.is_empty() => false,
            line if line.starts_with("#") => false,
            line if line.starts_with("---") => false,
            _ => true,
        })
        .map(|line| line.to_lowercase())
        .fold(HashMap::new(), |mut acc, line| {
            let first_char = line.remove_to().chars().next().unwrap();
            acc.entry(first_char).or_insert(Vec::new()).push(line);
            acc
        });

    // println!("{:#?}", grouped_by_first_letter);

    let mut output = format!("### Mobile\n\n\n### Laptop\n\n\n---\n\n");
    for first_letter in 'a'..='z' {
        let lines = grouped_by_first_letter.get_mut(&first_letter);
        if let Some(lines) = lines {
            lines.sort_by(|a, b| a.remove_to().cmp(b.remove_to()));
            let markdown_string = format!(
                "### {}\n{}\n\n",
                first_letter.to_uppercase().next().unwrap(),
                lines.join("\n")
            );
            output.push_str(&markdown_string)
        }
    }

    println!("{}", output);

    Ok(())
}

trait ParticleRemover {
    fn remove_to(&self) -> &str;
}

impl ParticleRemover for String {
    fn remove_to(&self) -> &str {
        if self.starts_with("to ") {
            &self[3..]
        } else {
            &self[..]
        }
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
