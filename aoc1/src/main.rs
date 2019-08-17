
use std::collections::HashSet;
use std::io::{self, Read, Write};
// This is called "boxing an error" -> https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
// basically it puts all your errors in a box, which is a way of storing a reference to a thing where you don't know how big it will be
type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

// This means
fn main() -> Result<()> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;

    // () is a value of the type () and its purpose is to be useless.
    // see https://stackoverflow.com/questions/24842271/what-is-the-purpose-of-the-unit-type-in-rust for more deets
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut freq = 0;
    for line in input.lines() {
        let change: i32 = line.parse().unwrap();
        freq += change;
    }

    // uses "Write" imported at top
    writeln!(io::stdout(), "{}", freq)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut freq = 0;
    // see https://doc.rust-lang.org/std/collections/struct.HashSet.html
    let mut record = HashSet::new();
    record.insert(0);

    // Why do I need this loop?
    loop {
        for line in input.lines() {
            let change: i32 = line.parse().unwrap();
            freq += change;
            if record.contains(&freq) {
                writeln!(io::stdout(), "{}", freq)?;
                return Ok(());
            }
            record.insert(freq);
        }
    }
}