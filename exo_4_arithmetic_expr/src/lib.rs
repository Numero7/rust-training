use std::{char, collections::HashMap};

pub enum ArithmeticExpr {
    // Define the structure of an arithmetic expression
    Number(i32),                                   // A constant number
    Add(Box<ArithmeticExpr>, Box<ArithmeticExpr>), // Addition
    Sub(Box<ArithmeticExpr>, Box<ArithmeticExpr>), // Subtraction
    Mul(Box<ArithmeticExpr>, Box<ArithmeticExpr>), // Multiplication
    Div(Box<ArithmeticExpr>, Box<ArithmeticExpr>), // Division
    Variable(char),                                // Free variable
}

impl ArithmeticExpr {
    pub fn from_rpn(rpn: &str) -> Self {
        let mut stack: Vec<ArithmeticExpr> = Vec::new();

        for token in rpn.split_whitespace() {
            match token {
                num if num.parse::<i32>().is_ok() => {
                    let number = num.parse::<i32>().unwrap();
                    stack.push(ArithmeticExpr::Number(number));
                }
                "+" | "-" | "*" | "/" => {
                    if stack.len() < 2 {
                        panic!("Invalid RPN: not enough operands for operator {}", token);
                    }

                    let right = Box::new(stack.pop().unwrap());
                    let left = Box::new(stack.pop().unwrap());

                    let expr = match token {
                        "+" => ArithmeticExpr::Add(left, right),
                        "-" => ArithmeticExpr::Sub(left, right),
                        "*" => ArithmeticExpr::Mul(left, right),
                        "/" => ArithmeticExpr::Div(left, right),
                        _ => unreachable!(), // This case is already handled above
                    };

                    stack.push(expr);
                }
                str if str.chars().count() == 1 => {
                    let ch = str.chars().next().expect("");
                    stack.push(ArithmeticExpr::Variable(ch));
                }
                _ => panic!("Invalid token in RPN: {}", token),
            }
        }
        if stack.len() != 1 {
            panic!("Invalid RPN: leftover elements in the stack");
        }
        stack.pop().unwrap()
    }

    pub fn size(&self) -> usize {
        //        todo!("Compute the size of the regex")
        match self {
            ArithmeticExpr::Number(_) => 1,
            ArithmeticExpr::Variable(_) => 1,
            ArithmeticExpr::Add(left, right) => 1 + left.size() + right.size(),
            ArithmeticExpr::Sub(left, right) => 1 + left.size() + right.size(),
            ArithmeticExpr::Mul(left, right) => 1 + left.size() + right.size(),
            ArithmeticExpr::Div(left, right) => 1 + left.size() + right.size(),
        }
    }

    pub fn evaluate(&self, vars: &HashMap<char, i32>) -> i32 {
        //todo!("Compute the value of the expression using the given values for free variables")
        match self {
            ArithmeticExpr::Number(x) => *x,
            ArithmeticExpr::Add(left, right) => left.evaluate(vars) + right.evaluate(vars),
            ArithmeticExpr::Sub(left, right) => left.evaluate(vars) - right.evaluate(vars),
            ArithmeticExpr::Mul(left, right) => left.evaluate(vars) * right.evaluate(vars),
            ArithmeticExpr::Div(left, right) => left.evaluate(vars) / right.evaluate(vars),
            ArithmeticExpr::Variable(x) => *vars.get(x).expect("Unknown free variableclear"),
        }
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
        let rpn = "3 x *";
        let expr = ArithmeticExpr::from_rpn(rpn);

        assert_eq!(expr.size(), 3);
        let mut vars = HashMap::default();
        vars.insert('x', 3);
        assert_eq!(expr.evaluate(&vars), 9);

        vars.insert('x', 5);
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
        let rpn = "x 1 + y + z 2 * *";
        let expr = ArithmeticExpr::from_rpn(rpn);

        let mut vars = HashMap::default();
        vars.insert('x', 8);
        vars.insert('y', 2);
        vars.insert('z', 5);
        assert_eq!(expr.evaluate(&vars), 110);

        vars.insert('x', 984);
        vars.insert('y', 17);
        vars.insert('z', 0);
        assert_eq!(expr.evaluate(&vars), 0);
    }
}
