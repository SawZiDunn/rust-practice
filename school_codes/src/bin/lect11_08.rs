use std::io::BufRead;

fn count_lines(pathname: &str) -> Result<(u32, u32), std::io::Error> {
    let f = std::fs::File::open(pathname)?;
    let f = std::io::BufReader::new(f);
    let mut n_lines = 0;
    let mut n_empty_lines = 0;

    for line in f.lines() {
        n_lines += 1;
        if line?.trim().len() == 0 {
            n_empty_lines += 1;
        }
    }

    Ok((n_lines, n_empty_lines))
}

fn main() {
    match count_lines("test.txt") {
        Ok((lines, empty)) => println!("Total {} lines , {} empty lines.", lines, empty),
        Err(e) => println!("Error: {}", e),
    }
}