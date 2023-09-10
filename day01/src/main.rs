use tools::file::read_input;

fn main() {
    let input = read_input("input/input.txt").unwrap();

    println!("Processing part 1...");
    process_part1(&input);

    println!("Processing part 2...");
    process_part2(&input);
}

fn process_part1(input: &String) -> Result<String, E> {
    for group in input
        .replace("\r\n", "\n\n")
        .split("\n\n") 
    {
        println!("New group");
        for line in group.lines() {
            let value = line.parse::<u64>()?;
            println!("{}", value);
        }
    }

    None
}

fn process_part2(input: &String) {
    //println!("{}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("input/test-input.txt").unwrap();
        let answer = process_part1(&input);
    }
}