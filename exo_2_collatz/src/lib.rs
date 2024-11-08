fn collatz_length(x: usize) -> usize {
    if x == 1 {
        return 0;
    } else if x % 2 == 0 {
        return 1 + collatz_length(x / 2);
    } else {
        return 1 + collatz_length(3 * x + 1);
    }
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
