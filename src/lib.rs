pub fn one() -> u32 {
    1
}

pub fn two() -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(one(), 1);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(), 2);
    }
}
