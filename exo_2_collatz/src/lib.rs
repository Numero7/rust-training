fn collatz_step(x: usize) -> usize {
    todo!("Return the next number in the collatz sequence")
}

fn collatz_length(x: usize) -> usize {
    todo!("Return the number of steps before reaching 1 in the collatz sequence starting from x")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collatz_1_is_zero() {
        let x = collatz_length(1);
        assert_eq!(x, 0);
    }

    #[test]
    fn collatz_2() {
        let x = collatz_length(2);
        assert_eq!(x, 1);
    }

    #[test]
    fn collatz_8() {
        let x = collatz_length(8);
        assert_eq!(x, 3);
    }

    #[test]
    fn collatz_27() {
        let x = collatz_length(27);
        assert_eq!(x, 111);
    }
}
