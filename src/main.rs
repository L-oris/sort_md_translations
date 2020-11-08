use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_contents = fs::read_to_string("sample.md")?;
    let mut clean_contents: Vec<&str> = raw_contents
        .lines()
        .filter(|&line| match line {
            line if line.is_empty() => false,
            line if line.starts_with("#") => false,
            line if line.starts_with("---") => false,
            _ => true,
        })
        .collect();

    clean_contents.sort();
    println!("{:?}", clean_contents);

    Ok(())
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
