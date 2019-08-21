use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();

    // takes arg we pass in after < and stores it in input
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    // we initialize an array of 256 unsigned 8-bit integers
    let mut frequencies = [0u8; 256];
    let (mut twos, mut threes) = (0, 0);

    for line in input.lines() {
        // run through our entire array and set them all to 0
        // in order to actually mutate the value, we use iter_mut()
        for f in frequencies.iter_mut() {
            *f = 0;
        }

        for b in line.as_bytes().iter().map(|&b| b as usize) {
            // increment the # in the array each time we see an ascii char
            // saturating_add will prevent going over the bounds of the data type,
            // so instead of 255 to -256 or something, you'll stay at 255
            frequencies[b] = frequencies[b].saturating_add(1);
        }

        // run through the array and see how many times two characters occurred in this line
        // add 1 to two's
        if frequencies.iter().any(|&f| f == 2) {
            twos += 1;
        }
        if frequencies.iter().any(|&f| f == 3) {
            threes += 1;
        }
    }
    writeln!(io::stdout(), "{}", twos * threes)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let ids: Vec<&str> = input.lines().collect();
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            if let Some(common) = common_correct_letters(&ids[i], &ids[j]) {
                writeln!(io::stdout(), "{}", common)?;
                return Ok(());
            }
        }
    }
    Err(From::from("could not find two correct box ids"))
}

fn common_correct_letters(id1: &str, id2: &str) -> Option<String> {
    if id1.len() != id2.len() {
        return None;
    }

    let mut found_one_wrong = false;
    // zip lets you iterate on two iterators
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 != c2 {
            if found_one_wrong {
                return None;
            }
            found_one_wrong = true;
        }
    }
    Some(
        id1.chars()
            .zip(id2.chars()) // so we have c1, c2
            .filter(|&(c1, c2)| c1 == c2) // cut out non-matches
            .map(|(c, _)| c) // map the common lettar
            .collect(), // consume the iterator to produce a string of thte common matches
    )
}