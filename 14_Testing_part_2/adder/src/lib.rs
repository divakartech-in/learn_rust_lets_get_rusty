pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Not equal"))
        }
    }

    #[test]
    fn it_works2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
