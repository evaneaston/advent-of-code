#[derive(Debug, Clone)]
pub(super) struct Assignment {
    pub(super) symbol: String,
    pub(super) expression: Expression,
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.symbol, self.expression)
    }
}

#[derive(Debug, Clone)]
pub(super) enum Expression {
    Number(i64),
    BinaryExpression(String, Operation, String),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::BinaryExpression(left, op, right) => {
                write!(f, "{} {} {}", left, op, right)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}",
            match self {
                Operation::Add => "+",
                Operation::Subtract => "-",
                Operation::Multiply => "*",
                Operation::Divide => "/",
            }
        )
    }
}

