use std::{collections::HashMap, ops::Mul};

pub enum ArithmeticExpr {
    // Define the structure of an arithmetic expression
}

impl ArithmeticExpr {
    pub fn from_rpn(rpn: &str) -> Self {
        todo!("Construct an arithmetic expression from a Reverse Polish Notation string")
    }

    pub fn size(&self) -> usize {
        todo!("Compute the size of the regex")
    }

    pub fn evaluate(&self, vars: &HashMap<usize, i32>) -> i32 {
        todo!("Compute the value of the expression using the given values for free variables")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_add() {
        let rpn = "3 4 +";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.size(), 3);
        assert_eq!(expr.evaluate(&HashMap::default()), 7);
    }

    #[test]
    fn mul_var() {
        let rpn = "3 x_1 *";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.size(), 3);
        let mut vars = HashMap::default();
        vars.insert(1, 3);
        assert_eq!(expr.evaluate(&vars), 9);

        vars.insert(1, 5);
        assert_eq!(expr.evaluate(&vars), 15);
    }

    #[test]
    fn sub() {
        let rpn = "7 9 -";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.evaluate(&HashMap::default()), -2);
    }

    #[test]
    fn depth_two() {
        let rpn = "7 9 - 4 +";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.evaluate(&HashMap::default()), 2);
    }

    #[test]
    fn odd_sum() {
        let rpn = "1 3 + 5 + 7 9 + 11 + +";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.size(), 11);
        assert_eq!(expr.evaluate(&HashMap::default()), 36);
    }

    #[test]
    fn many_variables() {
        let rpn = "x_1 1 + x_2 + x_3 2 * *";
        let expr = ArithmeticExpr::from_rpn(rpn);

        let mut vars = HashMap::default();
        vars.insert(1, 8);
        vars.insert(2, 2);
        vars.insert(3, 5);
        assert_eq!(expr.evaluate(&vars), 110);

        vars.insert(1, 984);
        vars.insert(2, 17);
        vars.insert(3, 0);
        assert_eq!(expr.evaluate(&vars), 0);
    }
}

trait Magma {}

impl<'a, T> Magma for &'a T where &'a T: Mul<Output = T> {}
