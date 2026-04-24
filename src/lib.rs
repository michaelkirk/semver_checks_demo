pub fn one() -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(one(), 1);
    }
}
