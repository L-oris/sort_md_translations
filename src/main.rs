use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string("sample.md")?;

    println!("{}", file_contents);

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
