// pub fn one() -> u64 {
//     1
// }

pub fn three() -> u32 {
    3
}

pub fn four() -> f64 {
    4.0
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_one() {
    //     assert_eq!(one(), 1);
    // }

    #[test]
    fn test_two() {
        assert_eq!(four(), 4.0);
    }
}
