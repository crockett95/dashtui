pub mod docset;
pub mod json;
pub mod stats;

pub fn sum_up_to(n: u32) -> u32 {
    let mut i = 0;
    let mut sum = 0;
    while i <= n {
        sum += i;
        i += 1;
    }
    sum
}

pub fn shout(say: &str) -> String {
    format!("{}!", say.to_uppercase())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_up_to() {
        assert_eq!(sum_up_to(0), 0);
        assert_eq!(sum_up_to(1), 1);
        assert_eq!(sum_up_to(3), 6);
        assert_eq!(sum_up_to(10), 55);
    }

    #[test]
    fn can_shout() {
        assert_eq!(shout("hello"), "HELLO!");
        assert_eq!(shout(""), "!");
        assert_eq!(shout("Rust"), "RUST!");
    }
}
