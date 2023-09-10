pub fn say_hi() {
    println!("Hi!");
}

pub fn read_input(path: &str) -> color_eyre::Result<String> {
    let input = std::fs::read_to_string(path)?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_works() {
        let result = read_input("src/test.txt").unwrap();
        assert_eq!(result, "Hello, world!\n");
    }
}